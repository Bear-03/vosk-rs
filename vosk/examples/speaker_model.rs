use hound::WavReader;
use vosk::{Model, Recognizer, SpeakerModel};

const MODEL_PATH: &str = "../stt-tts/resources/model";
const SPEAKER_MODEL_PATH: &str = "C:/Users/user/Downloads/spk-model";
const WAV_PATH: &str = "../stt-tests-two/test.wav";

fn main() {
    let mut reader = WavReader::open(WAV_PATH).unwrap();
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();

    let model = Model::new(MODEL_PATH).unwrap();
    let spk_model = SpeakerModel::new(SPEAKER_MODEL_PATH).unwrap();
    let mut recognizer =
        Recognizer::new_with_speaker(&model, reader.spec().sample_rate as f32, &spk_model).unwrap();

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
