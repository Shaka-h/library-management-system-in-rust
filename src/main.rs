use author::{Authors, Author};

mod author;
pub mod books;
mod patron;

fn main() {
    let mut author_one = Author::create_author(
        "John Doe".to_string(),
        "john.doe@example.com".to_string(),
        10,
        "USA".to_string(),
        1000,
    );

    let mut author_two = Author::create_author(
        "Miriam Shaka".to_string(),
        "shaka@example.com".to_string(),
        10,
        "TZ".to_string(),
        1000,
    );

    let mut authors_collection = Authors {
        author: Vec::new(),
    };

    authors_collection.add_author(&mut author_one);
    authors_collection.add_author(&mut author_two);

    println!("{:?}", authors_collection.author);

    authors_collection.edit_author(&2, &"Doe".to_string());

    authors_collection.vote_for_author(&1);

    println!("{:?}", authors_collection.author);

    


}
