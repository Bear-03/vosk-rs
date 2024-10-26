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
