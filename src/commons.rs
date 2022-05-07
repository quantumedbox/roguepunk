#[derive(Copy, Clone, Debug)]
pub enum CardinalDir {
    East,
    South,
    West,
    North,
}

// todo: Make ranges such as 33..=96 constant and reusable

impl CardinalDir {
    #[inline]
    pub fn from_circle_rotation(fraction: u8) -> Self {
        match fraction {
            225..=u8::MAX | 0..=32 => CardinalDir::East,
            33..=96 => CardinalDir::South,
            97..=160 => CardinalDir::West,
            161..=224 => CardinalDir::North,
        }
    }
}

// todo: SubcardinalDir -> Handy way of representing 8-way rotations

/// Used for propagating relations in unsigned coordinate system
#[derive(Copy, Clone)]
pub enum Relation<T> {
    Add(T),
    Sub(T),
}

impl<T> Relation<T> where T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> {
    #[inline]
    pub fn apply(&self, base: T) -> T {
        match self {
            Relation::Add(lhs) => base + *lhs,
            Relation::Sub(lhs) => base - *lhs,
        }
    }
}
