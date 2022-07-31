//! Run with:
//! cargo run --example grammar <model path> <wav path>
//! e.g. "cargo run --example grammar /home/user/stt/model /home/user/stt/test.wav"
//! (The WAV file must have signed 16-bit samples)
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
        .expect("A path for the WAV file to be read was not provided");

    let mut reader = WavReader::open(wav_path).expect("Could not create the WAV reader");
    let samples = reader
        .samples()
        .collect::<hound::Result<Vec<i16>>>()
        .expect("Could not read WAV file");

    let model = Model::new(model_path).expect("Could not create the model");

    let mut recognizer = Recognizer::new_with_grammar(
        &model,
        reader.spec().sample_rate as f32,
        // Provide a list of phrases to be recognized.
        //
        // If "[unk]" is added, it will be the fallback for any word that could not be recognized.
        // Otherwise, the best match will be used in the result, even if it is most likely
        // incorrect.
        //
        // Note that the words in a phrase can still be recognized separately
        &["one two three four five six seven eight nine zero", "[unk]"],
    )
    .expect("Could not create the recognizer");

    for sample in samples.chunks(4000) {
        let state = recognizer.accept_waveform(sample);
        match state {
            DecodingState::Finalized => {
                println!("{:#?}", recognizer.result().single().unwrap());
            }
            DecodingState::Running => {
                println!("{:#?}", recognizer.partial_result());
            }
            DecodingState::Failed => {
                eprintln!("an error occurred")
            }
        }
    }

    println!("{:#?}", recognizer.final_result().single().unwrap());
}
