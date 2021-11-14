//! Contains the traits for our books.

use crate::{
    book, book_full, book_simple,
    book_type::{BookType, CoverType},
};

/// Gives you a quick overview of the book.
pub trait Blurb {
    /// Returns a quick overview of the book.
    fn blurb(&self) -> String;
}

/// Adds the possibility get a key to sort by.
pub trait SortKey {
    type T: PartialOrd;

    /// Returns the key to sort by.
    fn get_key(&self) -> Self::T;
}

impl Blurb for book_simple::Book {
    fn blurb(&self) -> String {
        match self.content.len() {
            0..=20 => format!("{}: {}", self.title, self.content),
            _ => format!("{}: {}...", self.title, self.content[..17].to_owned()),
        }
    }
}

impl Blurb for book::PublicBook {
    fn blurb(&self) -> String {
        let kind = match &self.kind {
            BookType::Physical(cover_type) => match cover_type {
                CoverType::Hardcover => "HC",
                CoverType::Softcover => "SC",
            },
            BookType::AudioBook => "A",
            BookType::EBook => "E",
        };
        match self.content.len() {
            0..=20 => format!("{} ({}): {}", self.title, kind, self.content),
            _ => format!(
                "{} ({}): {}...",
                self.title,
                kind,
                self.content[..17].to_owned()
            ),
        }
    }
}

impl Blurb for book_full::Book {
    fn blurb(&self) -> String {
        let kind = match self.kind() {
            BookType::Physical(cover_type) => match cover_type {
                CoverType::Hardcover => "HC",
                CoverType::Softcover => "SC",
            },
            BookType::AudioBook => "A",
            BookType::EBook => "E",
        };
        match self.content().len() {
            0..=20 => format!("{} ({}): {}", self.title(), kind, self.content()),
            _ => format!(
                "{} ({}): {}...",
                self.title(),
                kind,
                self.content()[..17].to_owned()
            ),
        }
    }
}

impl<'a> SortKey for &'a book_simple::Book {
    type T = (&'a str, &'a str);

    fn get_key(&self) -> Self::T {
        (&self.author, &self.title)
    }
}

impl<'a> SortKey for &'a book::PublicBook {
    type T = (&'a BookType, &'a str, &'a str);

    fn get_key(&self) -> Self::T {
        (&self.kind, &self.author, &self.title)
    }
}

impl<'a> SortKey for &'a book_full::Book {
    type T = (&'a BookType, &'a str, &'a str);

    fn get_key(&self) -> Self::T {
        (self.kind(), self.author(), self.title())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        book_full, book_simple,
        book_type::{BookType, CoverType},
    };

    #[test]
    fn blurb_book_full() {
        let books = book_full::Book::new(
            "Zebras in the wild",
            "12093488745",
            BookType::Physical(CoverType::Softcover),
            "Anu Tamm",
            "Some content here",
        );
        assert_eq!(
            books.blurb(),
            "Zebras in the wild (SC): Some content here".to_owned()
        );
    }

    #[test]
    fn sort_simple() {
        let books = vec![
            book_simple::Book {
                title: "Zebras in the wild".to_string(),
                isbn: "12093488745".to_string(),
                author: "Anu Tamm".to_string(),
                content: "Some content here".to_string(),
            },
            book_simple::Book {
                title: "Zebras in the wild".to_string(),
                isbn: "1923454999".to_string(),
                author: "Anu Tamm".to_string(),
                content: "Some content here".to_string(),
            },
            book_simple::Book {
                title: "An island of radiators".to_string(),
                isbn: "9516241165".to_string(),
                author: "Norman Torm".to_string(),
                content: "Some content here".to_string(),
            },
        ];
        assert!(std::ptr::eq(
            books.iter().max_by_key(|b| b.get_key()).unwrap(),
            &books[2]
        ));
    }
}
