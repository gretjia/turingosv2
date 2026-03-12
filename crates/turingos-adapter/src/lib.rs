pub mod boundary;
#[cfg(test)]
mod fixture;

pub use boundary::{AdapterError, AdapterOutcome, IntentAdapter};
