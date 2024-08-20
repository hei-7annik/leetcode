/// Explanation
/// Put books on a bookshelf with minimal height through optimal configuration of books on each shelf
///
/// # Example
/// ```rust
/// let books = [[1,3],[2,4],[3,2]]
///     .iter()
///     .map(|book| book.to_vec())
///     .collect();
///
/// assert_eq!(min_height_shelves(books, 6), 4);
/// ```
///
/// # Cases
/// 1. Shelf height does not increase with the new book
/// 2. Shelf height increase is minimal with the new book on a shelf with previous books
/// 3. Shelf height increase is minimal with the new book on a new shelf
///
/// # Method
/// 1. Treat each book like it's the last one.
/// 2. For each book: Find the selection of previous books resulting in minimal `shelf_height`.
/// 3. Add `shelf_height` to the `total_shelf_height` calculated for the bookcase, back when the
///    first book not on the shelf was the last book.
///

pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
    let mut total_shelf_height = Vec::from([0]);

    for i in 0..books.len() {
        let mut shelf_height = 0;
        let mut book_stack_width = 0;

        let height = total_shelf_height.iter()
            .zip(&books[..=i])
            .rev()
            .take_while(|(_, &ref book)| {
                // going back, select as many books as will fit on one shelf
                book_stack_width += book[0];
                book_stack_width <= shelf_width
            })
            .map( |(bookcase_height, book)| {
                shelf_height = i32::max(shelf_height, book[1]);

                bookcase_height + shelf_height
            }).min();

        total_shelf_height.push(height.unwrap());
    }

    total_shelf_height.pop().unwrap()
}