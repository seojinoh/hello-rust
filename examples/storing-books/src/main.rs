struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

impl Library {
    fn new() -> Library {
        Library {
            books: Vec::new()
        }
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        self.books.len() == 0
        // Solution
        // self.books.is_empty()
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn print_books(&self) {
        for book in &self.books {
            println!("┌ title: {}", book.title);
            println!("└ year: {}", book.year);
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        let mut oldest: Option<&Book> = None;

        for book in self.books.iter() {
            if oldest.is_none() || oldest.unwrap().year > book.year {
                oldest = Some(book);
            }
        }

        oldest
    }
}

fn main() {
    let mut library = Library::new();

    println!("The library is empty: library.is_empty() -> {}", library.is_empty());

    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

    println!("The library is no longer empty: library.is_empty() -> {}", library.is_empty());

    library.print_books();

    match library.oldest_book() {
        Some(book) => println!("The oldest book is {}", book.title),
        None => println!("The library is empty!"),
    }

    println!("The library has {} books", library.len());

    library.print_books();
}