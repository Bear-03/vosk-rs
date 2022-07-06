//! Run with:
//! cargo run --example microphone <model path> <duration>
//! e.g. "cargo run --example microphone /home/user/stt/model 10"
//!
//! Check out the "Run the examples" section in the README to know how to link the vosk dynamic
//! libaries to the examples

use std::{
    env,
    sync::{Arc, Mutex},
    time::Duration,
};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    ChannelCount,
};
use dasp::{sample::ToSample, Sample};
use vosk::{Model, Recognizer};

fn main() {
    let mut args = env::args();
    args.next();

    let model_path = args.next().expect("A model path was not provided");
    let record_duration = Duration::from_secs(
        args.next()
            .expect("A recording duration was not provided")
            .parse()
            .expect("Invalid recording duration"),
    );

    let audio_input_device = cpal::default_host()
        .default_input_device()
        .expect("No input device connected");

    let config = audio_input_device
        .default_input_config()
        .expect("Failed to load default input config");
    let channels = config.channels();

    let model = Model::new(model_path).expect("Could not create the model");
    let mut recognizer = Recognizer::new(&model, config.sample_rate().0 as f32)
        .expect("Could not create the Recognizer");

    recognizer.set_max_alternatives(10);
    recognizer.set_words(true);
    recognizer.set_partial_words(true);

    let recognizer = Arc::new(Mutex::new(recognizer));

    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };

    let recognizer_clone = recognizer.clone();
    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => audio_input_device.build_input_stream(
            &config.into(),
            move |data: &[f32], _| recognize(&mut recognizer_clone.lock().unwrap(), data, channels),
            err_fn,
        ),
        cpal::SampleFormat::U16 => audio_input_device.build_input_stream(
            &config.into(),
            move |data: &[u16], _| recognize(&mut recognizer_clone.lock().unwrap(), data, channels),
            err_fn,
        ),
        cpal::SampleFormat::I16 => audio_input_device.build_input_stream(
            &config.into(),
            move |data: &[i16], _| recognize(&mut recognizer_clone.lock().unwrap(), data, channels),
            err_fn,
        ),
    }
    .expect("Could not build stream");

    stream.play().expect("Could not play stream");
    println!("Recording...");

    std::thread::sleep(record_duration);
    drop(stream);

    println!("{:#?}", recognizer.lock().unwrap().final_result());
}

fn recognize<T: Sample + ToSample<i16>>(
    recognizer: &mut Recognizer,
    data: &[T],
    channels: ChannelCount,
) {
    let data: Vec<i16> = data.iter().map(|v| v.to_sample()).collect();
    let data = if channels != 1 {
        stereo_to_mono(&data)
    } else {
        data
    };

    let state = recognizer.accept_waveform(&data);
    match state {
        vosk::DecodingState::Running => {
            let partial_result = recognizer.partial_result();
            println!("partial: {:?}", partial_result);
        }
        vosk::DecodingState::Finalized => {
            let result = recognizer.result();
            result.multiple().map(|r| println!("result: {:?}", r));
        }
        vosk::DecodingState::Failed => {
            println!("error")
        }
    }
}

pub fn stereo_to_mono(input_data: &[i16]) -> Vec<i16> {
    let mut result = Vec::with_capacity(input_data.len() / 2);
    result.extend(
        input_data
            .chunks_exact(2)
            .map(|chunk| chunk[0] / 2 + chunk[1] / 2),
    );

    result
}
