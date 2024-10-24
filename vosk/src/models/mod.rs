#[cfg(feature = "batch")]
mod batch;
mod sequential;

#[cfg(feature = "batch")]
pub use batch::BatchModel;
pub use sequential::{Model, SpeakerModel};
