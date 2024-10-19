use derive_more::{Error, From, Into};
use derive_new::new;
use fmt_derive::Display;

#[derive(new, Error, Display, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct DuplicateOutcomeError {
    #[new(into)]
    outcome: String,
}

impl DuplicateOutcomeError {}
