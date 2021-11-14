//! Contains book types.

/// Represents the type of book.
#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Debug)]
pub enum BookType {
    /// A physical book made of paper and string and glue
    Physical(CoverType),
    /// An audiobook
    AudioBook,
    /// An e-book for e-readers
    EBook,
}

/// Represents the type of physical book.
#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Debug)]
pub enum CoverType {
    /// Hardcover books usually are more expensive.
    Hardcover,
    /// Softcover books bring reading to the masses.
    Softcover,
}
