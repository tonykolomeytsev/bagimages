use std::collections::HashMap;

use image::{ImageBuffer, RgbImage};
use rosbag::record_types::Connection;
use rosbag::{ChunkRecord, MessageRecord, RosBag};

use crate::common::cursor::Cursor;
use crate::common::error::AppError;
use crate::common::naming::to_res_name;
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

    /// Topic files base name
    pub res_name: String,
}

impl TopicState {
    fn new(name: String) -> Self {
        TopicState {
            counter: 0,
            extracted: 0,
            res_name: to_res_name(&name),
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

    for record in bag.chunk_records() {
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
        MessageRecord::Connection(connection) => process_connection(&connection, &topics, states)?,
        MessageRecord::MessageData(data) => {
            // Process message only if the data message was preceded by a connection message
            // Reading a message with connection will create an entry in states.
            if let Some(state) = states.get_mut(&data.conn_id) {
                state.counter += 1;

                // Export no more than `args.number` images
                match (args.number, state.extracted) {
                    (Some(number), extracted) if extracted >= number => return Ok(()),
                    _ => (),
                }

                // Export images not in a row, but through step
                if (state.counter - 1) % args.step != 0 {
                    return Ok(());
                }

                process_image(&args, state, &data.data)?;
            }
        }
    }
    renderer.render(&states, true);
    Ok(())
}

fn process_connection(
    connection: &Connection,
    topics: &[&str],
    states: &mut HashMap<u32, TopicState>,
) -> Result<(), AppError> {
    let conn_id = connection.id;
    let key = connection.topic;
    let is_desired_topic = topics.contains(&key);
    let is_desired_type = connection.tp == TOPIC_IMAGE_TYPE;

    match (is_desired_topic, is_desired_type) {
        (true, true) => {
            states.insert(conn_id, TopicState::new(key.to_string()));
            Ok(())
        }
        (true, false) => Err(AppError::RosBagInvalidTopicType(
            key.to_string(),
            TOPIC_IMAGE_TYPE.to_string(),
        )),
        _ => Ok(()),
    }
}

fn process_image(args: &Args, state: &mut TopicState, data: &[u8]) -> Result<(), AppError> {
    let image = sensor_msgs::Image::from_reader(&mut Cursor::new(data))?;

    let mut buffer: RgbImage =
        ImageBuffer::from_vec(image.width, image.height, image.data.to_vec())
            .ok_or(AppError::InvalidImageEncoding(image.encoding.to_string()))?;

    // for cases when cv_bridge shits yourself and mix up color channels
    if args.invert_channels {
        for p in buffer.pixels_mut() {
            let [r, g, b] = p.0;
            p.0 = [b, g, r];
        }
    }

    let save_path = format!(
        "{}/{}_{}.png",
        args.output_dir,
        state.res_name,
        state.extracted + 1,
    );

    buffer
        .save_with_format(&save_path, image::ImageFormat::Png)
        .map_err(|e| AppError::CannotSave(save_path, e.to_string()))?;

    state.extracted += 1;
    Ok(())
}
