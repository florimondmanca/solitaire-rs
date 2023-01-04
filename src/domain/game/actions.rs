use crate::domain::Board;

#[derive(Debug)]
pub enum Action<'a> {
    TargetPrevious(&'a Board),
    TargetNext(&'a Board),
    IncreaseRange(&'a Board),
    DecreaseRange,
    Build(&'a mut Board),
    Act(&'a mut Board),
    Discard(&'a mut Board),
}
