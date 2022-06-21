use std::collections::HashMap;
use std::io::Cursor;

use image::{ImageBuffer, RgbImage};
use rosbag::{ChunkRecord, MessageRecord, RosBag};

use crate::common::error::AppError;
use crate::features::extract::view::View;
use crate::sensor_msgs::{self};
use crate::{args::Args, features::renderer::Renderer};

const TOPIC_IMAGE_TYPE: &'static str = "sensor_msgs/Image";

#[derive(Debug, Clone)]
pub struct TopicState {
    /// Number of all encountered frames
    pub counter: u32,

    /// Number of all extracted frames
    pub extracted: u32,

    /// Topic name
    pub name: String,
}

impl TopicState {
    fn new(name: String) -> Self {
        TopicState {
            counter: 0,
            extracted: 0,
            name,
        }
    }
}

pub fn extract(args: Args) -> Result<(), AppError> {
    let renderer = Renderer();
    let bag = RosBag::new(&args.path_to_bag).map_err(|e| AppError::RosBagOpen(e.to_string()))?;

    let topics = args
        .topics
        .iter()
        .map(String::as_str)
        .collect::<Vec<&str>>();

    let mut states: HashMap<u32, TopicState> = HashMap::new();

    for record in bag.chunk_records()
    /* take */
    {
        if let Some(max) = args.number {
            let is_all_finished = states.iter().all(|(_, v)| v.extracted >= max);
            let is_all_found = states.len() == topics.len();
            if is_all_finished && is_all_found {
                break;
            }
        }

        match record.map_err(|e| AppError::RosBagInvalidChunk(e.to_string()))? {
            ChunkRecord::Chunk(chunk) => {
                for msg in chunk.messages() {
                    let msg = msg.map_err(|e| AppError::RosBagInvalidMesage(e.to_string()))?;

                    process_message(&args, msg, &mut states, &topics, &renderer)?;
                }
            }
            _ => (),
        }
    }

    renderer.render(&states, false);
    renderer.line(View::Done);
    Ok(())
}

fn process_message(
    args: &Args,
    msg: MessageRecord,
    states: &mut HashMap<u32, TopicState>,
    topics: &[&str],
    renderer: &Renderer,
) -> Result<(), AppError> {
    match msg {
        MessageRecord::Connection(connection) => {
            let conn_id = connection.id;
            let key = connection.topic;
            let is_desired_topic = topics.contains(&key);
            let is_desired_type = connection.tp == TOPIC_IMAGE_TYPE;

            match (is_desired_topic, is_desired_type) {
                (true, true) => {
                    states.insert(conn_id, TopicState::new(key.to_string()));
                }
                (true, false) => {
                    return Err(AppError::RosBagInvalidTopicType(
                        key.to_string(),
                        TOPIC_IMAGE_TYPE.to_string(),
                    ))
                }
                _ => (),
            };
        }
        MessageRecord::MessageData(data) => {
            let conn_id = data.conn_id;

            if let Some(state) = states.get_mut(&conn_id) {
                state.counter += 1;
                match (args.number, state.extracted) {
                    (Some(max), extracted) if extracted >= max => return Ok(()),
                    _ => (),
                }

                if state.counter % args.step != 0 {
                    return Ok(());
                }

                let mut cursor = Cursor::new(data.data);
                let msg_image = sensor_msgs::Image::from_reader(&mut cursor);

                let mut buffer: RgbImage =
                    ImageBuffer::from_vec(msg_image.width, msg_image.height, msg_image.data)
                        .unwrap();

                // IDK why blue and green channels are mixed
                for p in buffer.pixels_mut() {
                    let [r, g, b] = p.0;
                    p.0 = [r, b, g];
                }

                let save_path = format!(
                    "{}/{}_{}.png",
                    args.output_dir,
                    msg_image.header.frame_id,
                    state.extracted + 1,
                );
                buffer
                    .save_with_format(save_path, image::ImageFormat::Png)
                    .unwrap();
                state.extracted += 1;
            }
        }
    }
    renderer.render(&states, true);
    Ok(())
}
