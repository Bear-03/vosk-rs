//! Run with:
//! cargo run --example grammar <model path> <wav path>
//! e.g. "cargo run --example grammar /home/user/stt/model /home/user/stt/test.wav"
//!
//! Read the "Setup" section in the README to know how to link the vosk dynamic
//! libaries to the examples

use std::env;

use hound::WavReader;
use vosk::{DecodingState, Model, Recognizer};

fn main() {
    let mut args = env::args();
    args.next();

    let model_path = args.next().expect("A model path was not provided");
    let wav_path = args
        .next()
        .expect("A path for the wav file to be read was not provieded");

    let mut reader = WavReader::open(wav_path).expect("Could not create the WAV reader");
    let samples: Vec<i16> = reader.samples().filter_map(|s| s.ok()).collect();

    let model = Model::new(model_path).expect("Could not create the model");

    let mut recognizer = Recognizer::new_with_grammar(
        &model,
        reader.spec().sample_rate as f32,
        // Provide a (comma separated) list of phrases to be recognized.
        // Anything else will be returned as [unk]
        &[
            "oh one two three four five six seven eight nine zero",
            "[unk]",
        ],
    )
    .expect("Could not create the recognizer");

    for sample in samples.chunks(4000) {
        let stream = recognizer.accept_waveform(sample);
        match stream {
            DecodingState::Finalized => {
                println!("{:?}", recognizer.result().single());
            }
            DecodingState::Running => {
                println!("{:?}", recognizer.partial_result());
            }
            DecodingState::Failed => {
                println!("an error occurred")
            }
        }
    }

    println!("{:#?}", recognizer.final_result().single().unwrap());
}
