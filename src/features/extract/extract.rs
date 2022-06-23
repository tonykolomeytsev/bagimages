use std::collections::BTreeMap;

use image::{ImageBuffer, RgbImage};
use rosbag::record_types::Connection;
use rosbag::{ChunkRecord, MessageRecord, RosBag};

use crate::common::cursor::Cursor;
use crate::common::error::AppError;
use crate::common::naming::to_res_name;
use crate::features::extract::view::View;
use crate::sensor_msgs::{self};
use crate::{args::Args, features::renderer::Renderer};

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

    /// Is export process done?
    pub done: bool,
}

impl TopicState {
    fn new(name: String) -> Self {
        TopicState {
            counter: 0,
            extracted: 0,
            res_name: to_res_name(&name),
            name,
            done: false,
        }
    }
}

pub fn extract(args: Args) {
    let renderer = Renderer();
    renderer.new_line();

    if let Err(e) = extract_internal(args, &renderer) {
        renderer.line(View::Error(e.to_string()))
    }
}

pub fn extract_internal(args: Args, renderer: &Renderer) -> Result<(), AppError> {
    validate_args(&args, &renderer)?;

    let bag = RosBag::new(&args.path_to_bag).map_err(|e| AppError::RosBagOpen(e.to_string()))?;

    let topics = args
        .topics
        .iter()
        .map(String::as_str)
        .collect::<Vec<&str>>();

    let mut states: BTreeMap<u32, TopicState> = BTreeMap::new();
    let mut start_time: u64 = 0;

    for record in bag.chunk_records() {
        // Export process termination criteria
        let is_all_finished = states.iter().all(|(_, v)| v.done);
        let is_all_found = states.len() == topics.len();
        if is_all_finished && is_all_found {
            break;
        }

        match record.map_err(|e| AppError::RosBagInvalidChunk(e.to_string()))? {
            ChunkRecord::Chunk(chunk) => {
                for msg in chunk.messages() {
                    let msg = msg.map_err(|e| AppError::RosBagInvalidMesage(e.to_string()))?;
                    process_message(&args, msg, &mut states, &topics, &mut start_time, &renderer)?;
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
    states: &mut BTreeMap<u32, TopicState>,
    topics: &[&str],
    start_time: &mut u64,
    renderer: &Renderer,
) -> Result<(), AppError> {
    match msg {
        MessageRecord::Connection(connection) => process_connection(&connection, &topics, states)?,
        MessageRecord::MessageData(data) => {
            // Use first message time as start time
            if *start_time == 0u64 {
                *start_time = data.time;
            }
            // Process message only if the data message was preceded by a connection message
            // Reading a message with connection will create an entry in states.
            if let Some(state) = states.get_mut(&data.conn_id) {
                let elapsed_time_sec = (data.time - *start_time) as f64 / 1_000_000_000_f64;

                // Export images only after specified start time
                if elapsed_time_sec < args.start {
                    return Ok(());
                }

                // Export images only before specified end time
                if let Some(end_time) = args.end {
                    if elapsed_time_sec > end_time {
                        state.done = true;
                        return Ok(());
                    }
                }

                state.counter += 1;

                // Export no more than `args.number` images
                match (args.number, state.extracted) {
                    (Some(number), extracted) if extracted >= number => {
                        state.done = true;
                        return Ok(());
                    }
                    _ => (),
                }

                // Export images not in a row, but through step
                if (state.counter - 1) % args.step != 0 {
                    return Ok(());
                }

                // renderer.line(View::Info(format!("time {}\n", elapsed_time_sec)));
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
    states: &mut BTreeMap<u32, TopicState>,
) -> Result<(), AppError> {
    let conn_id = connection.id;
    let key = connection.topic;
    let is_desired_topic = topics.contains(&key);
    let is_desired_type = connection.tp == sensor_msgs::Image::ros_type();

    match (is_desired_topic, is_desired_type) {
        (true, true) => {
            states.insert(conn_id, TopicState::new(key.to_string()));
            Ok(())
        }
        (true, false) => Err(AppError::RosBagInvalidTopicType(
            key.to_string(),
            sensor_msgs::Image::ros_type().to_string(),
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

fn validate_args(args: &Args, renderer: &Renderer) -> Result<(), AppError> {
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!("input rosbag file: {}", args.path_to_bag));
    lines.push(format!("output dir: {}", args.output_dir));

    if args.topics.is_empty() {
        return Err(AppError::ArgsEmptyTopics);
    }

    match (args.start, args.end) {
        // time start or end specified with negative values
        (start, Some(end)) if start < 0f64 || end < 0f64 => {
            return Err(AppError::ArgsNegativeTime(start, end))
        }

        // time end lower than or equals time start
        (start, Some(end)) if end <= start => return Err(AppError::ArgsEndBeforeStart(start, end)),

        // default start time is 0
        (start, Some(end)) if start == 0f64 => {
            lines.push(format!("export from bag start until the {:.} sec", end))
        }

        // non-default start and end time
        (start, Some(end)) => lines.push(format!(
            "export from {:.} sec until the {:.} sec",
            start, end,
        )),

        // default start and end time
        (start, None) if start == 0f64 => lines.push(format!("export from start until the end")),

        // non-default start time
        (start, None) => lines.push(format!("export from {:.} sec until the end", start)),
    }

    match (args.number, args.step) {
        // negative step
        (_, step) if step < 1 => return Err(AppError::ArgsNegativeStep(step)),

        // negative frames number
        (Some(number), _) if number < 1 => return Err(AppError::ArgsNegativeNumber(number)),

        // frames number and step are not specified
        (None, step) if step == 1 => lines.push("export every frame".to_string()),

        // frames number is not specified, step is specified
        (None, step) => lines.push(format!("export every {}-th frame", step)),

        // frames number is specified, step is not specified
        (Some(number), _) if number == 1 => lines.push(format!("export only one frame per topic")),

        // frames number and step are specified
        (Some(number), step) if step == 1 => {
            lines.push(format!("export {} frames per topic", number))
        }

        // frames number and step are specified
        (Some(number), step) => lines.push(format!(
            "export every {}-th frame, {} frames per topic",
            step, number,
        )),
    }

    if args.invert_channels {
        lines.push("invert color channels (RGB8 to BGR8 and vice-versa)".to_string());
    }

    renderer.line(View::RunningExport(lines));
    Ok(())
}
