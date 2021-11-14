//! Contains an intermediate book implementation.
use crate::book_type::BookType;

/// Represents a book.
///
/// # Example
///
/// ```rust
/// use intro_to_rust::{book::PublicBook, book_type::{BookType, CoverType}};
///
/// let book = PublicBook {
///     title: "Top secret report".to_owned(),
///     isbn: "REDACTED".to_owned(),
///     kind: BookType::Physical(CoverType::Softcover),
///     author: "Unknown".to_owned(),
///     content: "REDACTED".to_owned()
/// };
///
/// assert_eq!(book.title, "Top secret report".to_owned());
/// let cover_type = match book.kind {
///     BookType::Physical(cover_type) => cover_type,
///     _ => panic!("Unknown book type!"),
/// };
/// match cover_type {
///     CoverType::Softcover => println!("Yay. Softcover secret reports are just my thing!"),
///     CoverType::Hardcover => panic!("It is supposed to be softcover! Back to the printshop, {}!", book.author)
/// };
/// ```
#[derive(Debug)]
pub struct PublicBook {
    pub title: String,
    pub isbn: String,
    pub kind: BookType,
    pub author: String,
    pub content: String,
}
