use crate::{book};

/// Autographs the content of the book.
///
/// This is an example of a modifying API. The book is borrowed and modified. The borrow is ended
/// after the function ends.
pub fn autograph(book: &mut book::PublicBook) {
    book.content = format!("{}!!! {}", book.author, book.content);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::book::PublicBook;
    use crate::book_type::BookType;

    #[test]
    fn sign_book() {
        let mut book = PublicBook {
            title: "Glassblowing for beginners".to_string(),
            isbn: "1951311699".to_string(),
            kind: BookType::EBook,
            author: "Edward Hothands".to_string(),
            content: "This is the content of the book here".to_string()
        };

        autograph(&mut book);

        assert_eq!(book.content, "Edward Hothands!!! This is the content of the book here".to_owned())
    }
}
