# 0.3.1
* Fix flag-enabled items not showing up on [docs.rs](https://docs.rs/vosk/0.3.0/vosk/index.html).

# 0.3.0
* Add support for Batch recognition ([PR](https://github.com/Bear-03/vosk-rs/pull/8)).
* [BREAKING] Redesign `LogLevel` to adequately represent Kaldi log levels ([PR](https://github.com/Bear-03/vosk-rs/pull/9)).
* [BREAKING] `Recognizer::accept_waveform` methods now return `Result<T, AcceptWaveformError>` (previously `T`).
  Vosk takes the buffer length as an `i32` so the user should be able to handle errors that arise due to the
  buffer being longer than `i32::MAX`.
* [BREAKING] `Model::find_word` now returns `Option<u32>` (previously `Option<u16>`) to adjust it to the values
  that Vosk can return.

# 0.2.0
* Documentation fixes.
* Loosen bounds for Recognizer::new_with_grammar.
* [BREAKING] Extra double quotes are no longer needed for phrases in `Recognizer::new_with_grammar`.

# 0.1.0
* First release.

