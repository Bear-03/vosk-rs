use crate::{Model, SpeakerModel};

use serde::Deserialize;
use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_int},
    ptr::NonNull,
};
use vosk_sys::*;

pub use results::*;

mod results;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// State of the decodification after processing a chunk of data.
pub enum DecodingState {
    /// Silence has occured and you can retrieve a new utterance with the [`Recognizer::result`].
    Finalized,
    /// Decoding still continues.
    Running,
    /// Decoding failed in some way.
    Failed,
}

impl DecodingState {
    /// Returns the variant that corresponds to `value` in C.
    pub(self) fn from_c_int(value: c_int) -> Self {
        match value {
            1 => Self::Finalized,
            0 => Self::Running,
            _ => Self::Failed,
        }
    }
}

/// C function that returns any type of result from the recognizer (partial, "regular" or final).
type ResultFn = unsafe extern "C" fn(*mut VoskRecognizer) -> *const c_char;

/// The main object which processes data.
/// Takes audio as input and returns decoded information as words, confidences, times, and other metadata.
pub struct Recognizer(NonNull<VoskRecognizer>);

impl Recognizer {
    /// Creates the recognizer object. Returns [`None`] if a problem occured.
    ///
    /// The recognizers process the speech and return text using shared model data.
    ///
    /// * `model` - [`Model`] containing static data for recognizer. Model can be shared
    /// across recognizers, even running in different threads.
    ///
    /// * `sample_rate` - The sample rate of the audio you going to feed into the recognizer.
    /// Make sure this rate matches the audio content, it is a common issue causing accuracy problems.
    ///
    /// [`Model`]: crate::Model
    #[must_use]
    pub fn new(model: &Model, sample_rate: f32) -> Option<Self> {
        let recognizer_ptr = unsafe { vosk_recognizer_new(model.0.as_ptr(), sample_rate) };
        Some(Self(NonNull::new(recognizer_ptr)?))
    }

    /// Creates the recognizer object with speaker recognition. Returns [`None`] if a problem occured
    ///
    /// With the speaker recognition mode the recognizer not just recognize
    /// text but also return speaker vectors one can use for speaker identification
    ///
    /// * `model` - [`Model`] containing the data for recognizer. Model can be
    /// shared across recognizers, even running in different threads.
    ///
    /// * `sample_rate` - The sample rate of the audio you going to feed into the recognizer.
    /// Make sure this rate matches the audio content, it is a common
    /// issue causing accuracy problems.
    ///
    /// * `spk_model` - Speaker model for speaker identification.
    ///
    /// [`Model`]: crate::Model
    #[must_use]
    pub fn new_with_speaker(
        model: &Model,
        sample_rate: f32,
        speaker_model: &SpeakerModel,
    ) -> Option<Self> {
        let recognizer_ptr = unsafe {
            vosk_recognizer_new_spk(model.0.as_ptr(), sample_rate, speaker_model.0.as_ptr())
        };

        Some(Self(NonNull::new(recognizer_ptr)?))
    }

    /// Creates the recognizer object with that only recognizes certain words.
    /// Returns [`None`] if a problem occured.
    ///
    /// Sometimes when you want to improve recognition accuracy and when you don't need
    /// to recognize large vocabulary you can specify a list of phrases to recognize. This
    /// will improve recognizer speed and accuracy but might return \[unk\] if user said
    /// something different.
    ///
    /// Only recognizers with lookahead models support this type of quick configuration.
    /// Precompiled HCLG graph models are not supported.
    ///
    /// * `model` - [`Model`] containing the data for recognizer. Model can be shared
    /// across recognizers, even running in different threads.
    ///
    /// * `sample_rate` - The sample rate of the audio you going to feed into the recognizer.
    /// Make sure this rate matches the audio content, it is a common issue causing accuracy problems.
    ///
    /// * `grammar` - The list of phrases to recognize.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use vosk::{Model, Recognizer};
    /// #
    /// let model = Model::new("/path/to/model").expect("Could not create a model");
    /// let recognizer = Recognizer::new_with_grammar(
    ///     &model,
    ///     16000.0,
    ///     &["one two three four five", "[unk]"],
    /// )
    /// .expect("Could not create a recognizer");
    /// ```
    ///
    /// [`Model`]: crate::Model
    #[must_use]
    pub fn new_with_grammar(
        model: &Model,
        sample_rate: f32,
        grammar: &[impl AsRef<str>],
    ) -> Option<Self> {
        let grammar_c = CString::new(format!(
            "[{}]",
            grammar
                .iter()
                .map(|phrase| format!("\"{}\"", phrase.as_ref()))
                .collect::<Vec<_>>()
                .join(", ")
        ))
        .ok()?;
        let recognizer_ptr =
            unsafe { vosk_recognizer_new_grm(model.0.as_ptr(), sample_rate, grammar_c.as_ptr()) };

        Some(Self(NonNull::new(recognizer_ptr)?))
    }

    /// Adds speaker model to already initialized recognizer
    ///
    /// Can add speaker recognition model to already created recognizer. Helps to initialize
    /// speaker recognition for grammar-based recognizer.
    pub fn set_speaker_model(&mut self, speaker_model: &SpeakerModel) {
        unsafe { vosk_recognizer_set_spk_model(self.0.as_ptr(), speaker_model.0.as_ptr()) }
    }

    /// Configures recognizer to output n-best results in [`result`] and [`final_result`]
    ///
    /// The returned value from those methods will be a [`CompleteResult::Single`]
    /// if `max_alternatives` is 0, and [`CompleteResult::Multiple`] otherwise.
    ///
    /// * `max_alternatives` - Maximum alternatives to return (may be fewer) (default: 0)
    ///
    /// [`result`]: Self::result
    /// [`final_result`]: Self::final_result
    /// [`CompleteResult::Single`]: crate::CompleteResult::Single
    /// [`CompleteResult::Multiple`]: crate::CompleteResult::Multiple
    pub fn set_max_alternatives(&mut self, max_alternatives: u16) {
        unsafe { vosk_recognizer_set_max_alternatives(self.0.as_ptr(), max_alternatives as c_int) }
    }

    /// Enables or disables words with metadata in the output, represented as:
    ///
    /// * [`WordInAlternative`] in a [`CompleteResult::Multiple`]
    ///
    /// * [`Word`] in a [`CompleteResult::Single`]
    ///
    /// [`WordInAlternative`]: crate::WordInAlternative
    /// [`Word`]: crate::Word
    /// [`CompleteResult::Multiple`]: crate::CompleteResult::Multiple
    /// [`CompleteResult::Single`]: crate::CompleteResult::Single
    pub fn set_words(&mut self, enable: bool) {
        unsafe { vosk_recognizer_set_words(self.0.as_ptr(), i32::from(enable)) }
    }

    /// Like [`set_words`], but for [`PartialResult`].
    ///
    /// Words will always be represented as [`Word`]
    ///
    /// [`set_words`]: Self::set_words
    /// [`PartialResult`]: crate::PartialResult
    /// [`Word`]: crate::Word
    pub fn set_partial_words(&mut self, enable: bool) {
        unsafe { vosk_recognizer_set_partial_words(self.0.as_ptr(), i32::from(enable)) }
    }

    /// Enables or disables Natural Language Semantics Markup Language (NLSML) in the output
    pub fn set_nlsml(&mut self, enable: bool) {
        unsafe { vosk_recognizer_set_nlsml(self.0.as_ptr(), i32::from(enable)) }
    }

    /// Accept and process new chunk of voice data.
    ///
    /// * `data` - Audio data in PCM 16-bit mono format.
    ///
    /// Returns a [`DecodingState`], which represents the state of the decodification
    /// after this chunk of data has been processed.
    pub fn accept_waveform(&mut self, data: &[i16]) -> DecodingState {
        // vosk_recognizer_accept_waveform and vosk_recognizer_accept_waveform_f are meant
        // to be used by languages that do not have an i16 type (those functions also take PCM 16-bit audio,
        // but represented as an f32 or i8). Those extra functions aren't needed in rust so they
        // will not be wrapped

        let decoding_state = unsafe {
            vosk_recognizer_accept_waveform_s(self.0.as_ptr(), data.as_ptr(), data.len() as i32)
        };

        DecodingState::from_c_int(decoding_state)
    }

    /// Returns speech recognition result, waiting for silence (see [`DecodingState::Finalized`]) to give a result.
    ///
    /// The returned value will be a [`CompleteResult::Single`]
    /// if [`set_max_alternatives`] was passed a 0 (default), and
    /// [`CompleteResult::Multiple`] otherwise.
    ///
    /// If words are enabled (see [`set_words`]), it also returns metadata abut the words.
    ///
    /// [`set_max_alternatives`]: Self::set_max_alternatives
    /// [`set_words`]: Self::set_words
    /// [`CompleteResult::Multiple`]: crate::CompleteResult::Multiple
    /// [`CompleteResult::Single`]: crate::CompleteResult::Single
    #[must_use]
    pub fn result(&mut self) -> CompleteResult {
        self.result_with_function(vosk_recognizer_result)
    }

    /// Returns partial speech recognition, which is not yet finalized and may change after
    /// processing more data.
    ///
    /// If words are enabled (see [`set_partial_words`]), it also returns metadata abut the words.
    ///
    /// [`set_partial_words`]: Self::set_partial_words
    #[must_use]
    pub fn partial_result(&mut self) -> PartialResult {
        self.result_with_function(vosk_recognizer_partial_result)
    }

    /// Returns speech recognition result. Like [`result`] but it does not
    /// wait for silence and it flushes the data so everything is processed
    ///
    /// [`result`]: Self::result
    #[must_use]
    pub fn final_result(&mut self) -> CompleteResult {
        self.result_with_function(vosk_recognizer_final_result)
    }

    /// Generic function to retrieve a given type of result from the recognizer.
    fn result_with_function<'de, T: Deserialize<'de>>(&mut self, function: ResultFn) -> T {
        // Panics in the result functions will never be the caller's fault, but rather some
        // edge case that was not thought of, so it does not make sense to return a Result.

        serde_json::from_str(
            unsafe { CStr::from_ptr(function(self.0.as_ptr())) }
                .to_str()
                .unwrap(),
        )
        .unwrap()
    }

    /// Resets current results and data so the recognition can continue from scratch
    pub fn reset(&mut self) {
        unsafe {
            vosk_recognizer_reset(self.0.as_ptr());
        }
    }
}

// SAFETY: Recognizer shares no state, so it is Send
unsafe impl Send for Recognizer {}
// SAFETY: All methods that mutate Recognizer require a &mut to it,
// which ensures exclusive access, so it is Sync
unsafe impl Sync for Recognizer {}

impl Drop for Recognizer {
    fn drop(&mut self) {
        unsafe { vosk_recognizer_free(self.0.as_ptr()) }
    }
}

#[cfg(feature = "cuda")]
pub mod batch_recognizer {
    use crate::batch_model::BatchModel;
    use vosk_sys::*;

    use std::{ffi::CStr, ptr::NonNull};

    pub use crate::recognition::results::*;

    /// The main object which processes data using GPU inferencing.
    /// Takes audio as input and returns decoded information as words, confidences, times, and other metadata.

    pub struct BatchRecognizer(std::ptr::NonNull<VoskBatchRecognizer>);

    impl BatchRecognizer {
        /// Creates the recognizer object. Returns [`None`] if a problem occured.
        ///
        /// The recognizers process the speech and return text using shared model data.
        ///
        /// * `model` - [`BatchModel`] containing static data for recognizer. Model can be shared
        /// across recognizers, even running in different threads.
        ///
        /// * `sample_rate` - The sample rate of the audio you going to feed into the recognizer.
        /// Make sure this rate matches the audio content, it is a common issue causing accuracy problems.
        ///
        /// [`BatchModel`]: crate::BatchModel
        #[must_use]
        pub fn new(model: &BatchModel, sample_rate: f32) -> Option<Self> {
            let recognizer_ptr =
                unsafe { vosk_batch_recognizer_new(model.0.as_ptr(), sample_rate) };
            Some(Self(NonNull::new(recognizer_ptr)?))
        }

        /// Enables or disables Natural Language Semantics Markup Language (NLSML) in the output
        pub fn set_nlsml(&mut self, enable: bool) {
            unsafe { vosk_batch_recognizer_set_nlsml(self.0.as_ptr(), i32::from(enable)) }
        }

        /// Accept and process new chunk of voice data.
        ///
        /// * `data` - Audio data in PCM 16-bit mono format as an array of i8.
        pub fn accept_waveform(&mut self, data: &[i8]) {
            unsafe {
                vosk_batch_recognizer_accept_waveform(
                    self.0.as_ptr(),
                    data.as_ptr(),
                    data.len() as i32,
                )
            };
        }

        /// Closes the stream to the model
        pub fn finish_stream(&mut self) {
            unsafe { vosk_batch_recognizer_finish_stream(self.0.as_ptr()) };
        }

        /// Gets the front of the result queue
        pub fn front_result(&mut self) -> Result<Word, serde_json::Error> {
            serde_json::from_str(
                unsafe { CStr::from_ptr(vosk_batch_recognizer_front_result(self.0.as_ptr())) }
                    .to_str()
                    .unwrap(),
            )
        }

        /// Removes the front of the result queue
        pub fn pop(&mut self) {
            unsafe { vosk_batch_recognizer_pop(self.0.as_ptr()) }
        }

        /// Gets the number of chunks that have yet to be processed
        pub fn get_pending_chunks(&mut self) -> i32 {
            (unsafe { vosk_batch_recognizer_get_pending_chunks(self.0.as_ptr()) }) as i32
        }
    }

    impl Drop for BatchRecognizer {
        fn drop(&mut self) {
            unsafe { vosk_batch_recognizer_free(self.0.as_ptr()) }
        }
    }
}
