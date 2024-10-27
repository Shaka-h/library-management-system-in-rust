use std::{ptr::null, sync::atomic::{AtomicUsize, Ordering}};

use crate::{author::Author, books::{Book, BookStatus}};

#[derive(Debug)]
pub struct Patron {
    id: usize,
    pub name: String,
    location: String,
}


pub struct BorrowedBooks<'a> {  // Add a lifetime parameter to Authors
    pub borrowed_books: Vec<&'a mut Book>,  // Use the same lifetime here
}

static PATRON_COUNTER: AtomicUsize = AtomicUsize::new(1);

impl Patron {
    pub fn add_patron (name: String, location: String) -> Patron {
        Patron {
            id: PATRON_COUNTER.fetch_add(1, Ordering::SeqCst),  // Get the next ID and increment the counter
            name,
            location,
        }
    }
}

impl <'a>BorrowedBooks<'a> {

    pub fn borrow_book (&mut self, book: &Book, patron: &mut Patron){
        if book.status == BookStatus::CheckedOut {
            println!("Book not Available");
        }

        else {
            book.status = BookStatus::Available;
            book.borrower = patron;
        }
    }

    pub fn return_book (&mut self, book: &Book, patron: Patron){
        if book.status == BookStatus::CheckedOut {
            println!("Book already Returned");
        }

        else {
            book.status = BookStatus::CheckedOut;
            book.borrower = null();
        }
    }

}