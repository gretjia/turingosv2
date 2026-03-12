#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StepIndex(pub u64);

impl StepIndex {
    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TraceHash(String);

impl TraceHash {
    pub fn genesis() -> Self {
        Self("GENESIS".to_owned())
    }

    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
