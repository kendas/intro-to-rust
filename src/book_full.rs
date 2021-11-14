//! Contains a more complex representation of a book with behaviors and encapsulation.
use crate::book_type::BookType;

/// Represents a book.
#[derive(Debug)]
pub struct Book {
    title: String,
    isbn: String,
    kind: BookType,
    author: String,
    content: String,
}

impl Book {
    /// Constructs a new book.
    ///
    /// # Example
    ///
    /// ```rust
    /// use intro_to_rust::book_full::Book;
    /// use intro_to_rust::book_type::{BookType, CoverType};
    ///
    /// let book = Book::new(
    ///     "Introduction to rust",
    ///     "947478288134",
    ///     BookType::Physical(CoverType::Softcover),
    ///     "Unknown",
    ///     "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
    /// );
    ///
    /// assert_eq!(book.title(), "Introduction to rust");
    /// assert_eq!(book.isbn(), "947478288134");
    /// ```
    pub fn new(title: &str, isbn: &str, kind: BookType, author: &str, content: &str) -> Book {
        Book {
            title: title.to_owned(),
            isbn: isbn.to_owned(),
            kind,
            author: author.to_owned(),
            content: content.to_owned(),
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn isbn(&self) -> &str {
        &self.isbn
    }

    pub fn kind(&self) -> &BookType {
        &self.kind
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}
