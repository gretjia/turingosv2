#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PredicateVerdict {
    Pass,
    Fail(Vec<String>),
}

impl PredicateVerdict {
    pub fn pass() -> Self {
        Self::Pass
    }

    pub fn fail(reasons: Vec<String>) -> Self {
        Self::Fail(reasons)
    }

    pub fn passed(&self) -> bool {
        matches!(self, Self::Pass)
    }

    pub fn reasons(&self) -> &[String] {
        match self {
            Self::Pass => &[],
            Self::Fail(reasons) => reasons,
        }
    }
}
