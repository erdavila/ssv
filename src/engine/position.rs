pub struct Position {
    pub line_number: usize,
    pub column_number: usize,
}

pub struct WithPosition<T> {
    pub value: T,
    pub position: Position,
}
