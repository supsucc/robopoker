pub(crate) trait Bucket: Sized + Copy + Eq + std::hash::Hash {}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub(crate) enum B {
    P1,
    P2,
    Ignore,
}
