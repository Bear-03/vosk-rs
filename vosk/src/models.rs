use std::{ffi::CString, ptr::NonNull};
use vosk_sys::*;

// SAFETY:
// All models can be safely shared across threads
// They hold static data and they won't be mutated by Vosk, so it is safe
// to pass ther pointer to multiple Recognizers even from immutable references
// https://github.com/alphacep/vosk-api/blob/a7bc5a22d411e22bebf4df1cc5554b473c7456d4/src/vosk_api.h

/// Model that stores all the data required for recognition.
pub struct Model(pub(crate) NonNull<VoskModel>);

impl Model {
    /// Loads model data from the file and returns the model object, or [`None`]
    /// if a problem occured.
    ///
    /// * `model_path` - the path to the model directory.
    #[must_use]
    pub fn new(model_path: impl Into<String>) -> Option<Self> {
        let model_path_c = CString::new(model_path.into()).ok()?;
        let model_ptr = unsafe { vosk_model_new(model_path_c.as_ptr()) };

        Some(Self(NonNull::new(model_ptr)?))
    }

    /// Check if a word can be recognized by the model.
    /// If it is, this returns Some with the index of the word in the model.
    /// If it is not, this returns None.
    ///
    /// Word symbol `0` is for `<epsilon>`.
    #[must_use]
    pub fn find_word(&mut self, word: &str) -> Option<u16> {
        let word_c = CString::new(word).ok()?;

        let symbol = unsafe { vosk_model_find_word(self.0.as_ptr(), word_c.as_ptr()) };

        if symbol == -1 {
            None
        } else {
            Some(symbol as u16)
        }
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        unsafe { vosk_model_free(self.0.as_ptr()) }
    }
}

unsafe impl Send for Model {}
unsafe impl Sync for Model {}

/// The same as [`Model`] but contains the data for speaker identification.
pub struct SpeakerModel(pub(crate) NonNull<VoskSpkModel>);

impl SpeakerModel {
    /// Loads speaker model data from the file and returns the model
    /// object, or [`None`] if a problem occured.
    ///
    /// * `model_path` - the path to the model in the filesystem.
    #[must_use]
    pub fn new(model_path: impl Into<String>) -> Option<Self> {
        let model_path_c = CString::new(model_path.into()).ok()?;
        let model_ptr = unsafe { vosk_spk_model_new(model_path_c.as_ptr()) };

        Some(Self(NonNull::new(model_ptr)?))
    }
}

impl Drop for SpeakerModel {
    fn drop(&mut self) {
        unsafe { vosk_spk_model_free(self.0.as_ptr()) }
    }
}

unsafe impl Send for SpeakerModel {}
unsafe impl Sync for SpeakerModel {}

#[cfg(feature = "cuda")]
pub mod batch_model {
    use std::{ffi::CString, ptr::NonNull};
    use vosk_sys::*;
    /// The same as [`Model`], but uses a CUDA enabled Nvidia GPU and dynamic batching to enable higher throughput.

    pub struct BatchModel(pub(crate) NonNull<VoskBatchModel>);

    impl BatchModel {
        /// Loads model data from the file and returns the model object, or [`None`]
        /// if a problem occured.
        ///
        /// * `model_path` - the path to the model directory.
        #[must_use]
        pub fn new(model_path: impl Into<String>) -> Option<Self> {
            let model_path_c = CString::new(model_path.into()).ok()?;
            let model_ptr = unsafe { vosk_batch_model_new(model_path_c.as_ptr()) };

            Some(Self(NonNull::new(model_ptr)?))
        }

        /// Waits for inferencing to finish
        pub fn wait(&self) {
            unsafe { vosk_batch_model_wait(self.0.as_ptr()) };
        }
    }

    impl Drop for BatchModel {
        fn drop(&mut self) {
            unsafe { vosk_batch_model_free(self.0.as_ptr()) }
        }
    }

    unsafe impl Send for BatchModel {}

    unsafe impl Sync for BatchModel {}
}
