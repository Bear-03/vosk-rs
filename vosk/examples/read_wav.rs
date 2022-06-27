use hound::WavReader;
use vosk::{DecodingState, Model, Recognizer};

const MODEL_PATH: &str = "../stt-tts/resources/model";
const WAV_PATH: &str = "audio.wav";

fn main() {
    let mut reader = WavReader::open(WAV_PATH).unwrap();
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();

    let model = Model::new(MODEL_PATH).unwrap();
    let mut recognizer = Recognizer::new(&model, reader.spec().sample_rate as f32).unwrap();

    recognizer.set_max_alternatives(10);
    recognizer.set_words(true);
    recognizer.set_partial_words(true);

    for sample in samples.chunks(100) {
        let decoding_state = recognizer.accept_waveform(sample);

        if decoding_state == DecodingState::Completed {
            println!("{:#?}", recognizer.result());
        } else {
            println!("{:#?}", recognizer.partial_result());
        }
    }

    println!("{:#?}", recognizer.final_result().multiple().unwrap());
}
