use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
/// A single word in a [`CompleteResultSingle`] and metadata about it.
///
/// Unlike in [`WordInAlternative`], the confidence ([`conf`]) is part of each word,
/// rather than part of an [`Alternative`].
///
/// [`conf`]: Self::conf
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

#[derive(Debug, Clone, Deserialize)]
/// A single word in an [`Alternative`] and metadata about it.
///
/// Unlike [`Word`], it does not contain the confidence,
/// as it is part of the [`Alternative`] itself.
pub struct WordInAlternative<'a> {
    /// Time in seconds when the word starts.
    pub start: f32,
    /// Time in seconds when the word ends.
    pub end: f32,
    /// The transcribed word.
    pub word: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
/// An alternative transcript in a [`CompleteResultMultiple`].
pub struct Alternative<'a> {
    /// Confidence of the recognizer that this is the correct alternative transcript.
    pub confidence: f32,
    #[serde(default)]
    /// Collection of words present in [`text`] with metadata about them.
    ///
    /// Empty unless [`Recognizer::set_words`] is set to true.
    ///
    /// [`Recognizer::set_words`]: crate::Recognizer::set_words
    /// [`text`]: Self::text
    pub result: Vec<WordInAlternative<'a>>,
    /// Full transcript text.
    pub text: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
/// Recognition result if [`Recognizer::set_max_alternatives`]
/// is set to a non-zero value.
///
/// Inner type of [`CompleteResult::Multiple`].
///
/// [`Recognizer::set_max_alternatives`]: crate::Recognizer::set_max_alternatives
pub struct CompleteResultMultiple<'a> {
    #[serde(borrow)]
    /// All the possible results of the transcription, ordered from most to less likely.
    pub alternatives: Vec<Alternative<'a>>,
}

#[derive(Debug, Clone, Deserialize)]
/// Recognition result if [`Recognizer::set_max_alternatives`]
/// is set to zero.
///
/// Inner type of [`CompleteResult::Single`].
///
/// [`Recognizer::set_max_alternatives`]: crate::Recognizer::set_max_alternatives
pub struct CompleteResultSingle<'a> {
    #[serde(default)]
    /// Collection of words present in [`text`] with metadata about them.
    ///
    /// Empty unless [`Recognizer::set_words`] is set to `true`.
    ///
    /// [`text`]: Self::text
    /// [`Recognizer::set_words`]: crate::Recognizer::set_words
    pub result: Vec<Word<'a>>,
    /// Full text of the transcript.
    pub text: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
/// Different results that can be returned from
/// [`Recognizer::result`] and [`Recognizer::final_result`].
///
/// [`Recognizer::result`]: crate::Recognizer::result
/// [`Recognizer::final_result`]: crate::Recognizer::final_result
pub enum CompleteResult<'a> {
    #[serde(borrow)]
    /// Result if [`Recognizer::set_max_alternatives`] is set to zero (default).
    ///
    /// [`Recognizer::set_max_alternatives`]: crate::Recognizer::set_max_alternatives
    Single(CompleteResultSingle<'a>),
    /// Result if [`Recognizer::set_max_alternatives`] is set to a non-zero value.
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

#[derive(Debug, Clone, Deserialize)]
/// Result returned by [`Recognizer::partial_result`].
/// The result may change after processing more data as decoding is not yet complete.
///
/// [`Recognizer::partial_result`]: crate::Recognizer::partial_result
pub struct PartialResult<'a> {
    // The "partial" JSON key will not be present if partial_result is called when the recognizer isn't running (DecodingState::Running).
    // It makes sense to return an empty string in that case
    #[serde(default)]
    /// Full text of the partial transcript.
    pub partial: &'a str,
    /// Collection of words present in [`partial`] with metadata about them.
    ///
    /// Empty unless [`Recognizer::set_partial_words`] is set to `true`.
    ///
    /// [`partial`]: Self::partial
    /// [`Recognizer::set_partial_words`]: crate::Recognizer::set_partial_words
    pub partial_result: Option<Vec<Word<'a>>>,
}
