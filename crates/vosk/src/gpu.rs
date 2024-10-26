/// Init, automatically select a CUDA device and allow multithreading.
/// Must be called once from the main thread.
pub fn gpu_init() {
    unsafe { vosk_sys::vosk_gpu_init() }
}

/// Init CUDA device in a multi-threaded environment.
/// Must be called for each thread.
pub fn gpu_thread_init() {
    unsafe { vosk_sys::vosk_gpu_thread_init() }
}
