#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub line_number: usize,
    pub column_number: usize,
}

#[derive(Debug)]
pub struct WithPosition<T> {
    pub value: T,
    pub position: Position,
}
