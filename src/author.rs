use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub struct Author {
    id: usize,
    pub name: String,
    pub email: String,
    pub n0_books: u32,
    pub country: String,
    pub votes: u32,
}

pub struct Authors<'a> {  // Add a lifetime parameter to Authors
    pub author: Vec<&'a mut Author>,  // Use the same lifetime here
}
static AUTHOR_COUNTER: AtomicUsize = AtomicUsize::new(1);

impl Author {
    pub fn create_author (name: String, email: String, n0_books: u32, country: String, votes: u32) -> Author {
        Author {
            id: AUTHOR_COUNTER.fetch_add(1, Ordering::SeqCst),  // Get the next ID and increment the counter
            name,
            email,
            n0_books,
            country,
            votes,
        }
    }

}

impl<'a> Authors<'a> {

    pub fn add_author(&mut self, author:&'a mut Author) {
        self.author.push(author);
    }

    pub fn edit_author(&mut self, id: &usize, new_name: &String) {
        for author in &mut self.author {
            println!("{:?}", author);
            if author.id == *id {
                author.name = new_name.to_string();
            }
            println!("{:?}", author);
        }
    }

    pub fn vote_for_author(&mut self, id: &usize) {
        if let Some(author) = self.author.iter_mut().find(|author| author.id == *id) {
            author.votes += 1;
        }
    }

}