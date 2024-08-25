use std::collections::VecDeque;

/// Explanation
/// Put books on a bookshelf with minimal height through choosing optimal selection of which books go on
/// each shelf
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

pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: usize) -> i32 {
    let mut bookshelf_heights = VecDeque::with_capacity(shelf_width + 1);
    bookshelf_heights.push_front(0);

    for i in 0..books.len() {
        let mut additional_shelf_height = 0;
        let mut width = 0;

        let height = bookshelf_heights.iter()
            .zip(books[..=i].iter().rev())
            .take_while(|(_, &ref book)| {
                // going back, select as many books as will fit on one shelf
                width += book[0];
                width <= shelf_width as i32
            })
            .map( |(bookshelf_height, book)| {
                additional_shelf_height = i32::max(additional_shelf_height, book[1]);
                bookshelf_height + additional_shelf_height
            }).min();

        bookshelf_heights.push_front(height.unwrap());
        bookshelf_heights.truncate(shelf_width);
    }
    *bookshelf_heights.front().unwrap()
}