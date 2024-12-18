/* automatically generated by rust-bindgen 0.60.1 */

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VoskModel {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VoskSpkModel {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VoskRecognizer {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VoskBatchModel {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VoskBatchRecognizer {
    _unused: [u8; 0],
}

#[cfg_attr(not(target_os = "windows"), link(name = "vosk"))]
#[cfg_attr(target_os = "windows", link(name = "libvosk"))]
extern "C" {
    #[doc = " Loads model data from the file and returns the model object"]
    #[doc = ""]
    #[doc = " @param model_path: the path of the model on the filesystem"]
    #[doc = " @returns model object or NULL if problem occured"]
    pub fn vosk_model_new(model_path: *const ::std::os::raw::c_char) -> *mut VoskModel;

    #[doc = " Releases the model memory"]
    #[doc = ""]
    #[doc = "  The model object is reference-counted so if some recognizer"]
    #[doc = "  depends on this model, model might still stay alive. When"]
    #[doc = "  last recognizer is released, model will be released too."]
    pub fn vosk_model_free(model: *mut VoskModel);

    #[doc = " Check if a word can be recognized by the model"]
    #[doc = " @param word: the word"]
    #[doc = " @returns the word symbol if @param word exists inside the model"]
    #[doc = " or -1 otherwise."]
    #[doc = " Reminding that word symbol 0 is for \\<epsilon\\>"]
    pub fn vosk_model_find_word(
        model: *mut VoskModel,
        word: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    #[doc = " Loads speaker model data from the file and returns the model object"]
    #[doc = ""]
    #[doc = " @param model_path: the path of the model on the filesystem"]
    #[doc = " @returns model object or NULL if problem occured"]
    pub fn vosk_spk_model_new(model_path: *const ::std::os::raw::c_char) -> *mut VoskSpkModel;

    #[doc = " Releases the model memory"]
    #[doc = ""]
    #[doc = "  The model object is reference-counted so if some recognizer"]
    #[doc = "  depends on this model, model might still stay alive. When"]
    #[doc = "  last recognizer is released, model will be released too."]
    pub fn vosk_spk_model_free(model: *mut VoskSpkModel);

    #[doc = " Creates the recognizer object"]
    #[doc = ""]
    #[doc = "  The recognizers process the speech and return text using shared model data"]
    #[doc = "  @param model       VoskModel containing static data for recognizer. Model can be"]
    #[doc = "                     shared across recognizers, even running in different threads."]
    #[doc = "  @param sample_rate The sample rate of the audio you going to feed into the recognizer."]
    #[doc = "                     Make sure this rate matches the audio content, it is a common"]
    #[doc = "                     issue causing accuracy problems."]
    #[doc = "  @returns recognizer object or NULL if problem occured"]
    pub fn vosk_recognizer_new(model: *mut VoskModel, sample_rate: f32) -> *mut VoskRecognizer;

    #[doc = " Creates the recognizer object with speaker recognition"]
    #[doc = ""]
    #[doc = "  With the speaker recognition mode the recognizer not just recognize"]
    #[doc = "  text but also return speaker vectors one can use for speaker identification"]
    #[doc = ""]
    #[doc = "  @param model       VoskModel containing static data for recognizer. Model can be"]
    #[doc = "                     shared across recognizers, even running in different threads."]
    #[doc = "  @param sample_rate The sample rate of the audio you going to feed into the recognizer."]
    #[doc = "                     Make sure this rate matches the audio content, it is a common"]
    #[doc = "                     issue causing accuracy problems."]
    #[doc = "  @param spk_model speaker model for speaker identification"]
    #[doc = "  @returns recognizer object or NULL if problem occured"]
    pub fn vosk_recognizer_new_spk(
        model: *mut VoskModel,
        sample_rate: f32,
        spk_model: *mut VoskSpkModel,
    ) -> *mut VoskRecognizer;

    #[doc = " Creates the recognizer object with the phrase list"]
    #[doc = ""]
    #[doc = "  Sometimes when you want to improve recognition accuracy and when you don't need"]
    #[doc = "  to recognize large vocabulary you can specify a list of phrases to recognize. This"]
    #[doc = "  will improve recognizer speed and accuracy but might return \\[unk\\] if user said"]
    #[doc = "  something different."]
    #[doc = ""]
    #[doc = "  Only recognizers with lookahead models support this type of quick configuration."]
    #[doc = "  Precompiled HCLG graph models are not supported."]
    #[doc = ""]
    #[doc = "  @param model       VoskModel containing static data for recognizer. Model can be"]
    #[doc = "                     shared across recognizers, even running in different threads."]
    #[doc = "  @param sample_rate The sample rate of the audio you going to feed into the recognizer."]
    #[doc = "                     Make sure this rate matches the audio content, it is a common"]
    #[doc = "                     issue causing accuracy problems."]
    #[doc = "  @param grammar The string with the list of phrases to recognize as JSON array of strings,"]
    #[doc = "                 for example \"\\[\"one two three four five\", \"\\[unk\\]\"\\]\"."]
    #[doc = ""]
    #[doc = "  @returns recognizer object or NULL if problem occured"]
    pub fn vosk_recognizer_new_grm(
        model: *mut VoskModel,
        sample_rate: f32,
        grammar: *const ::std::os::raw::c_char,
    ) -> *mut VoskRecognizer;

    #[doc = " Adds speaker model to already initialized recognizer"]
    #[doc = ""]
    #[doc = " Can add speaker recognition model to already created recognizer. Helps to initialize"]
    #[doc = " speaker recognition for grammar-based recognizer."]
    #[doc = ""]
    #[doc = " @param spk_model Speaker recognition model"]
    pub fn vosk_recognizer_set_spk_model(
        recognizer: *mut VoskRecognizer,
        spk_model: *mut VoskSpkModel,
    );

    #[doc = " Configures recognizer to output n-best results"]
    #[doc = ""]
    #[doc = " <pre>"]
    #[doc = "   {"]
    #[doc = "      \"alternatives\": ["]
    #[doc = "          { \"text\": \"one two three four five\", \"confidence\": 0.97 },"]
    #[doc = "          { \"text\": \"one two three for five\", \"confidence\": 0.03 },"]
    #[doc = "      ]"]
    #[doc = "   }"]
    #[doc = " </pre>"]
    #[doc = ""]
    #[doc = " @param max_alternatives - maximum alternatives to return from recognition results"]
    pub fn vosk_recognizer_set_max_alternatives(
        recognizer: *mut VoskRecognizer,
        max_alternatives: ::std::os::raw::c_int,
    );

    #[doc = " Enables words with times in the output"]
    #[doc = ""]
    #[doc = " <pre>"]
    #[doc = "   \"result\" : [{"]
    #[doc = "       \"conf\" : 1.000000,"]
    #[doc = "       \"end\" : 1.110000,"]
    #[doc = "       \"start\" : 0.870000,"]
    #[doc = "       \"word\" : \"what\""]
    #[doc = "     }, {"]
    #[doc = "       \"conf\" : 1.000000,"]
    #[doc = "       \"end\" : 1.530000,"]
    #[doc = "       \"start\" : 1.110000,"]
    #[doc = "       \"word\" : \"zero\""]
    #[doc = "     }, {"]
    #[doc = "       \"conf\" : 1.000000,"]
    #[doc = "       \"end\" : 1.950000,"]
    #[doc = "       \"start\" : 1.530000,"]
    #[doc = "       \"word\" : \"zero\""]
    #[doc = "     }, {"]
    #[doc = "       \"conf\" : 1.000000,"]
    #[doc = "       \"end\" : 2.340000,"]
    #[doc = "       \"start\" : 1.950000,"]
    #[doc = "       \"word\" : \"zero\""]
    #[doc = "     }, {"]
    #[doc = "       \"conf\" : 1.000000,"]
    #[doc = "       \"end\" : 2.610000,"]
    #[doc = "       \"start\" : 2.340000,"]
    #[doc = "       \"word\" : \"one\""]
    #[doc = "     }],"]
    #[doc = " </pre>"]
    #[doc = ""]
    #[doc = " @param words - boolean value"]
    pub fn vosk_recognizer_set_words(recognizer: *mut VoskRecognizer, words: ::std::os::raw::c_int);

    #[doc = " Like above return words and confidences in partial results"]
    #[doc = ""]
    #[doc = " @param partial_words - boolean value"]
    pub fn vosk_recognizer_set_partial_words(
        recognizer: *mut VoskRecognizer,
        partial_words: ::std::os::raw::c_int,
    );

    #[doc = " Set NLSML output"]
    #[doc = " @param nlsml - boolean value"]
    pub fn vosk_recognizer_set_nlsml(recognizer: *mut VoskRecognizer, nlsml: ::std::os::raw::c_int);

    #[doc = " Accept voice data"]
    #[doc = ""]
    #[doc = "  accept and process new chunk of voice data"]
    #[doc = ""]
    #[doc = "  @param data - audio data in PCM 16-bit mono format"]
    #[doc = "  @param length - length of the audio data"]
    #[doc = "  @returns 1 if silence is occured and you can retrieve a new utterance with result method"]
    #[doc = "           0 if decoding continues"]
    #[doc = "           -1 if exception occured"]
    pub fn vosk_recognizer_accept_waveform(
        recognizer: *mut VoskRecognizer,
        data: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = " Same as above but the version with the short data for language bindings where you have"]
    #[doc = "  audio as array of shorts"]
    pub fn vosk_recognizer_accept_waveform_s(
        recognizer: *mut VoskRecognizer,
        data: *const ::std::os::raw::c_short,
        length: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = " Same as above but the version with the float data for language bindings where you have"]
    #[doc = "  audio as array of floats"]
    pub fn vosk_recognizer_accept_waveform_f(
        recognizer: *mut VoskRecognizer,
        data: *const f32,
        length: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = " Returns speech recognition result"]
    #[doc = ""]
    #[doc = " @returns the result in JSON format which contains decoded line, decoded"]
    #[doc = "          words, times in seconds and confidences. You can parse this result"]
    #[doc = "          with any json parser"]
    #[doc = ""]
    #[doc = " <pre>"]
    #[doc = "  {"]
    #[doc = "    \"text\" : \"what zero zero zero one\""]
    #[doc = "  }"]
    #[doc = " </pre>"]
    #[doc = ""]
    #[doc = " If alternatives enabled it returns result with alternatives, see also vosk_recognizer_set_alternatives()."]
    #[doc = ""]
    #[doc = " If word times enabled returns word time, see also vosk_recognizer_set_word_times()."]
    pub fn vosk_recognizer_result(recognizer: *mut VoskRecognizer)
        -> *const ::std::os::raw::c_char;

    #[doc = " Returns partial speech recognition"]
    #[doc = ""]
    #[doc = " @returns partial speech recognition text which is not yet finalized."]
    #[doc = "          result may change as recognizer process more data."]
    #[doc = ""]
    #[doc = " <pre>"]
    #[doc = " {"]
    #[doc = "    \"partial\" : \"cyril one eight zero\""]
    #[doc = " }"]
    #[doc = " </pre>"]
    pub fn vosk_recognizer_partial_result(
        recognizer: *mut VoskRecognizer,
    ) -> *const ::std::os::raw::c_char;

    #[doc = " Returns speech recognition result. Same as result, but doesn't wait for silence"]
    #[doc = "  You usually call it in the end of the stream to get final bits of audio. It"]
    #[doc = "  flushes the feature pipeline, so all remaining audio chunks got processed."]
    #[doc = ""]
    #[doc = "  @returns speech result in JSON format."]
    pub fn vosk_recognizer_final_result(
        recognizer: *mut VoskRecognizer,
    ) -> *const ::std::os::raw::c_char;

    #[doc = " Resets the recognizer"]
    #[doc = ""]
    #[doc = "  Resets current results so the recognition can continue from scratch"]
    pub fn vosk_recognizer_reset(recognizer: *mut VoskRecognizer);

    #[doc = " Releases recognizer object"]
    #[doc = ""]
    #[doc = "  Underlying model is also unreferenced and if needed released"]
    pub fn vosk_recognizer_free(recognizer: *mut VoskRecognizer);

    #[doc = " Set log level for Kaldi messages"]
    #[doc = ""]
    #[doc = "  @param log_level the level"]
    #[doc = "     0 - default value to print info and error messages but no debug"]
    #[doc = "     less than 0 - don't print info messages"]
    #[doc = "     greather than 0 - more verbose mode"]
    pub fn vosk_set_log_level(log_level: ::std::os::raw::c_int);

    #[doc = "  Init, automatically select a CUDA device and allow multithreading."]
    #[doc = "  Must be called once from the main thread."]
    #[doc = "  Has no effect if HAVE_CUDA flag is not set."]
    pub fn vosk_gpu_init();

    #[doc = "  Init CUDA device in a multi-threaded environment."]
    #[doc = "  Must be called for each thread."]
    #[doc = "  Has no effect if HAVE_CUDA flag is not set."]
    pub fn vosk_gpu_thread_init();

    #[doc = " Creates the batch recognizer object"]
    #[doc = ""]
    #[doc = "  @returns model object or NULL if problem occured"]
    pub fn vosk_batch_model_new(model_path: *const ::std::os::raw::c_char) -> *mut VoskBatchModel;

    #[doc = " Releases batch model object"]
    pub fn vosk_batch_model_free(model: *mut VoskBatchModel);

    #[doc = " Wait for the processing"]
    pub fn vosk_batch_model_wait(model: *mut VoskBatchModel);

    #[doc = " Creates batch recognizer object"]
    #[doc = "  @returns recognizer object or NULL if problem occured"]
    pub fn vosk_batch_recognizer_new(
        model: *mut VoskBatchModel,
        sample_rate: f32,
    ) -> *mut VoskBatchRecognizer;

    #[doc = " Releases batch recognizer object"]
    pub fn vosk_batch_recognizer_free(recognizer: *mut VoskBatchRecognizer);

    #[doc = " Accept batch voice data"]
    pub fn vosk_batch_recognizer_accept_waveform(
        recognizer: *mut VoskBatchRecognizer,
        data: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
    );

    #[doc = " Set NLSML output"]
    #[doc = " @param nlsml - boolean value"]
    pub fn vosk_batch_recognizer_set_nlsml(
        recognizer: *mut VoskBatchRecognizer,
        nlsml: ::std::os::raw::c_int,
    );

    #[doc = " Closes the stream"]
    pub fn vosk_batch_recognizer_finish_stream(recognizer: *mut VoskBatchRecognizer);

    #[doc = " Return results"]
    pub fn vosk_batch_recognizer_front_result(
        recognizer: *mut VoskBatchRecognizer,
    ) -> *const ::std::os::raw::c_char;

    #[doc = " Release and free first retrieved result"]
    pub fn vosk_batch_recognizer_pop(recognizer: *mut VoskBatchRecognizer);

    #[doc = " Get amount of pending chunks for more intelligent waiting"]
    pub fn vosk_batch_recognizer_get_pending_chunks(
        recognizer: *mut VoskBatchRecognizer,
    ) -> ::std::os::raw::c_int;
}
