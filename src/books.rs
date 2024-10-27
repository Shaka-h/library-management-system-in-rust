use std::{ptr::null, sync::atomic::{AtomicUsize, Ordering}};

use crate::patron::Patron;

#[derive(Debug)]
pub struct Book {
    id: usize,
    pub name: String,
    pub author: String,
    pub year: u16,
    pub copies: u32,
    pub votes: u32,
    pub status: BookStatus,
    pub borrower: Patron
}

#[derive(Debug)]
pub enum BookStatus {
    Available,
    CheckedOut
}

pub struct Books_Collection<'a> {  // Add a lifetime parameter to Authors
    pub book: Vec<&'a mut Book>,  // Use the same lifetime here
}
static BOOK_COUNTER: AtomicUsize = AtomicUsize::new(1);

impl Book {
    pub fn publish_book (name: String, author: String, year: u16, copies: u32, votes: u32) -> Book {
        Book {
            id: BOOK_COUNTER.fetch_add(1, Ordering::SeqCst),  // Get the next ID and increment the counter
            name,
            author,
            year,
            copies,
            votes,
            status: BookStatus::Available,
            borrower: null()
        }
    }

    pub fn like_book(&mut self) {
        self.votes += 1;
    }
}

impl<'a> Books_Collection<'a> {

    pub fn add_book(&mut self, book:&'a mut Book) {
        self.book.push(book);
    }

    pub fn edit_book(&mut self, id: &usize, new_name: &String) {
        for book in &mut self.book {
            println!("{:?}", book);
            if book.id == *id {
                book.name = new_name.to_string();
            }
            println!("{:?}", book);
        }
    }

    pub fn vote_for_book(&mut self, id: &usize) {
        if let Some(book) = self.book.iter_mut().find(|book| book.id == *id) {
            book.votes += 1;
        }
    }

}