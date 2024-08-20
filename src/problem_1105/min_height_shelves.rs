/// Explanation
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
/// 1. Does a book fit on the shelves
/// 2. Is a book higher than all other books on the shelves
///
/// # Method
/// 1. Split right before accumulated width exceeds `shelf_width`
/// 2. Identify the highest book in each shelf
/// 3. Optimize by moving high books onto previous/next shelve
///
/// Lower bound roundUp(sum(books.width) / shelf.width)
/// Upper bound sum(books)
///

pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
    let mut combined_shelf_height = Vec::from([0]);

    for i in 0..books.len() {
        let mut shelf_height = 0;
        let mut book_stack_width = 0;

        let new_height = combined_shelf_height
            .iter()
            .zip(&books[..=i])
            .rev()
            .take_while(|(_, &ref book)| {
                book_stack_width += book[0];
                book_stack_width <= shelf_width
            })
            .map( |(bookcase_height, book)| {
                shelf_height = i32::max(shelf_height, book[1]);
                bookcase_height + shelf_height
            }).min();

        combined_shelf_height.push(new_height.unwrap());
    }

    combined_shelf_height.pop().unwrap()
}