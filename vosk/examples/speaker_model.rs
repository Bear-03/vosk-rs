//! Run with:
//! cargo run --example read_wav <model path> <speaker model path> <wav path>
//! e.g. "cargo run --example speaker_model /home/user/stt/model /home/user/stt/speaker_model /home/user/stt/test.wav"
//! (The WAV file must have signed 16-bit sample)
//!
//! Read the "Setup" section in the README to know how to link the vosk dynamic
//! libaries to the examples

use std::env;

use hound::WavReader;
use vosk::{Model, Recognizer, SpeakerModel};

fn main() {
    let mut args = env::args();
    args.next();

    let model_path = args.next().expect("A model path was not provided");
    let speaker_model_path = args.next().expect("A speaker model path was not provided");
    let wav_path = args
        .next()
        .expect("A path for the WAV file to be read was not provided");

    let mut reader = WavReader::open(wav_path).expect("Could not create the WAV reader");
    let samples = reader
        .samples()
        .collect::<hound::Result<Vec<i16>>>()
        .expect("Could not read WAV file");

    let model = Model::new(model_path).expect("Could not create the model");
    let spk_model =
        SpeakerModel::new(speaker_model_path).expect("Could not create the speaker model");
    let mut recognizer =
        Recognizer::new_with_speaker(&model, reader.spec().sample_rate as f32, &spk_model)
            .expect("Could not create the recognizer");

    // Alternatives cannot be enabled as the Alternative objets do not contain the speaker info
    // recognizer.set_max_alternatives(10);

    // Words will remain disabled so the speaker data is more visible, though they could be enabled
    // recognizer.set_words(true);
    // recognizer.set_partial_words(true);

    for sample in samples.chunks(100) {
        recognizer.accept_waveform(sample);
        println!("{:#?}", recognizer.partial_result());
    }

    println!("{:#?}", recognizer.final_result().single().unwrap());
}
