#[cfg(feature = "cuda")]
mod batch;
mod sequential;

#[cfg(feature = "cuda")]
pub use batch::BatchModel;
pub use sequential::{Model, SpeakerModel};
