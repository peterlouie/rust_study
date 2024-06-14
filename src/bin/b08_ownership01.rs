struct Book {
    pages: i32,
    rating: i32,
}

fn display_page_count(book: &Book) {
    println!("pages = {:?}", book.pages);
}

fn display_rating(book: &Book) {
    println!("pages = {:?}", book.rating);
}

fn main() {
    let book = Book {
        pages: 5,
        rating: 9,
    };

    //will not work
    //need to be borrow
    //display_page_count(book);
    //display_rating(book);

    //this will not also work
    //when function already borrow the variable
    //need also to be borrow
    //display_rating(book);

    display_page_count(&book);
    display_rating(&book);

    //this one still work
    println!("{:?}", book.rating);
}
