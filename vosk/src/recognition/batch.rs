use crate::{models::BatchModel, recognition::results::*};
use vosk_sys::*;

use std::{ffi::CStr, ptr::NonNull};

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

// SAFETY: Recognizer shares no state, so it is Send
unsafe impl Send for BatchRecognizer {}
// SAFETY: All methods that mutate Recognizer require a &mut to it,
// which ensures exclusive access, so it is Sync
unsafe impl Sync for BatchRecognizer {}


impl Drop for BatchRecognizer {
    fn drop(&mut self) {
        unsafe { vosk_batch_recognizer_free(self.0.as_ptr()) }
    }
}