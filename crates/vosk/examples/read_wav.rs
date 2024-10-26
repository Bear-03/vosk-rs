//! Run with:
//! cargo run --example read_wav <model path> <wav path>
//! e.g. "cargo run --example read_wav /home/user/stt/model /home/user/stt/test.wav"
//! (The WAV file must have signed 16-bit samples)
//!
//! Read the "Setup" section in the README to know how to link the vosk dynamic
//! libaries to the examples

use std::env;

use hound::WavReader;
use vosk::{Model, Recognizer};

fn main() {
    let mut args = env::args();
    args.next();

    let model_path = args.next().expect("A model path was not provided");
    let wav_path = args
        .next()
        .expect("A path for the wav file to be read was not provided");

    let mut reader = WavReader::open(wav_path).expect("Could not create the WAV reader");
    let samples = reader
        .samples()
        .collect::<hound::Result<Vec<i16>>>()
        .expect("Could not read WAV file");

    let model = Model::new(model_path).expect("Could not create the model");
    let mut recognizer = Recognizer::new(&model, reader.spec().sample_rate as f32)
        .expect("Could not create the recognizer");

    recognizer.set_max_alternatives(10);
    recognizer.set_words(true);
    recognizer.set_partial_words(true);

    for sample in samples.chunks(100) {
        recognizer.accept_waveform(sample);
        println!("{:#?}", recognizer.partial_result());
    }

    println!("{:#?}", recognizer.final_result().multiple().unwrap());
}
