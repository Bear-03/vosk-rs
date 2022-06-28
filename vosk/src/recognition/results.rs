use serde::Deserialize;

/// A single word in a [`CompleteResultSingle`] and metadata about it.
///
/// Unlike in [`WordInAlternative`], the confidence ([`conf`]) is part of each word,
/// rather than part of an [`Alternative`].
///
/// [`conf`]: Self::conf
#[derive(Debug, Clone, Deserialize)]
pub struct Word<'a> {
    /// Confidence that this word is.
    pub conf: f32,
    /// Time in seconds when the word starts.
    pub start: f32,
    /// Time in seconds when the word ends.
    pub end: f32,
    /// The transcribed word.
    pub word: &'a str,
}

/// A single word in an [`Alternative`] and metadata about it.
///
/// Unlike [`Word`], it does not contain the confidence,
/// as it is part of the [`Alternative`] itself.
#[derive(Debug, Clone, Deserialize)]
pub struct WordInAlternative<'a> {
    /// Time in seconds when the word starts.
    pub start: f32,
    /// Time in seconds when the word ends.
    pub end: f32,
    /// The transcribed word.
    pub word: &'a str,
}

/// An alternative transcript in a [`CompleteResultMultiple`].
#[derive(Debug, Clone, Deserialize)]
pub struct Alternative<'a> {
    /// Confidence of the recognizer that this is the correct alternative transcript.
    pub confidence: f32,
    /// Collection of words present in [`text`] with metadata about them.
    ///
    /// Empty unless [`Recognizer::set_words`] is passed `true`.
    ///
    /// [`text`]: Self::text
    /// [`Recognizer::set_words`]: crate::Recognizer::set_words
    #[serde(default)]
    pub result: Vec<WordInAlternative<'a>>,
    /// Full transcript text.
    pub text: &'a str,
}

/// Recognition result if [`Recognizer::set_max_alternatives`]
/// is passed a non-zero value.
///
/// Inner type of [`CompleteResult::Multiple`].
///
/// [`Recognizer::set_max_alternatives`]: crate::Recognizer::set_max_alternatives
#[derive(Debug, Clone, Deserialize)]
pub struct CompleteResultMultiple<'a> {
    /// All the possible results of the transcription, ordered from most to less likely.
    #[serde(borrow)]
    pub alternatives: Vec<Alternative<'a>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpeakerInfo {
    /// Speaker vectors used for speaker identification.
    #[serde(rename = "spk")]
    pub vector: Vec<f32>,
    /// Data frames in which the speaker was not in silence.
    #[serde(rename = "spk_frames")]
    pub frames: u16,
}

/// Recognition result if [`Recognizer::set_max_alternatives`]
/// is passed a zero (default).
///
/// Inner type of [`CompleteResult::Single`].
///
/// [`Recognizer::set_max_alternatives`]: crate::Recognizer::set_max_alternatives
#[derive(Debug, Clone, Deserialize)]
pub struct CompleteResultSingle<'a> {
    /// Data useful for speaker identification
    ///
    /// Enabled if the [`Recognizer`] was passed a [`SpeakerModel`]  with
    /// [`Recognizer::new_with_speaker`] or [`Recognizer::set_speaker_model`],
    /// [`None`] otherwise
    ///
    /// [`SpeakerModel`]: crate::SpeakerModel
    /// [`Recognizer`]: crate::Recognizer
    /// [`Recognizer::new_with_speaker`]: crate::Recognizer::new_with_speaker
    /// [`Recognizer::set_speaker_model`]: crate::Recognizer::set_speaker_model
    #[serde(flatten)]
    pub speaker_info: Option<SpeakerInfo>,
    /// Collection of words present in [`text`] with metadata about them.
    ///
    /// Empty unless [`Recognizer::set_words`] is passed `true`.
    ///
    /// [`text`]: Self::text
    /// [`Recognizer::set_words`]: crate::Recognizer::set_words
    #[serde(default)]
    pub result: Vec<Word<'a>>,
    /// Full text of the transcript.
    pub text: &'a str,
}

/// Different results that can be returned from
/// [`Recognizer::result`] and [`Recognizer::final_result`].
///
/// [`Recognizer::result`]: crate::Recognizer::result
/// [`Recognizer::final_result`]: crate::Recognizer::final_result
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum CompleteResult<'a> {
    /// Result if [`Recognizer::set_max_alternatives`] is passed zero (default).
    ///
    /// [`Recognizer::set_max_alternatives`]: crate::Recognizer::set_max_alternatives
    #[serde(borrow)]
    Single(CompleteResultSingle<'a>),
    /// Result if [`Recognizer::set_max_alternatives`] is passed a non-zero value.
    ///
    /// [`Recognizer::set_max_alternatives`]: crate::Recognizer::set_max_alternatives
    Multiple(CompleteResultMultiple<'a>),
}

impl<'a> CompleteResult<'a> {
    /// Returns the inner [`CompleteResultSingle`] if `self` was [`Single`], and [`None`] otherwise.
    ///
    /// [`Single`]: Self::Single
    pub fn single(self) -> Option<CompleteResultSingle<'a>> {
        match self {
            Self::Single(x) => Some(x),
            Self::Multiple(_) => None,
        }
    }

    /// Returns the inner [`CompleteResultMultiple`] if `self` was [`Multiple`], and [`None`] otherwise.
    ///
    /// [`Multiple`]: Self::Multiple
    pub fn multiple(self) -> Option<CompleteResultMultiple<'a>> {
        match self {
            Self::Single(_) => None,
            Self::Multiple(x) => Some(x),
        }
    }
}

/// Result returned by [`Recognizer::partial_result`].
/// The result may change after processing more data as decoding is not yet complete.
///
/// [`Recognizer::partial_result`]: crate::Recognizer::partial_result
#[derive(Debug, Clone, Deserialize)]
pub struct PartialResult<'a> {
    // The "partial" JSON key will not be present if partial_result is called when the recognizer isn't running (DecodingState::Running).
    // It makes sense to return an empty string in that case
    /// Full text of the partial transcript.
    #[serde(default)]
    pub partial: &'a str,
    /// Collection of words present in [`partial`] with metadata about them.
    ///
    /// Empty unless [`Recognizer::set_partial_words`] is passed `true`.
    ///
    /// [`partial`]: Self::partial
    /// [`Recognizer::set_partial_words`]: crate::Recognizer::set_partial_words
    #[serde(default)]
    pub partial_result: Vec<Word<'a>>,
}
