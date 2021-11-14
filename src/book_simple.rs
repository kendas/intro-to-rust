//! Contains a simple representation of a book.

/// The simplest way to represent a book.
///
/// # Example
///
/// ```rust
/// use intro_to_rust::book_simple::Book;
///
/// let book = Book {
///     title: "Introduction to rust".to_owned(),
///     isbn: "947478288134".to_owned(),
///     author: "Unknown".to_owned(),
///     content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_owned()
/// };
/// assert_eq!(book.title, "Introduction to rust".to_owned());
/// ```
#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub isbn: String,
    pub author: String,
    pub content: String,
}
