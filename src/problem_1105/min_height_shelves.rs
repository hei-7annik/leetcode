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
/// 1. Does a book fit in the shelf
/// 2. Is a book higher than all other books on the shelf
///
/// # Method
/// 1. Split right before accumulated width exceeds `shelf_width`
/// 2. Identify the highest book in each shelf
/// 3. Optimize by moving high books onto previous/next shelf
///
/// Lower bound roundUp(sum(books.width) / shelf.width)
/// Upper bound sum(books)
///
pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
    let mut current_shelf_capacity= shelf_width as u32;
    let mut current_shelf_height: i32 = 0;
    let mut combined_shelf_height: i32 = 0;


    books.iter().for_each(|book| {
        let thickness = book[0] as u32;
        let height = book[1];

        if let Some(reduced_capacity) = current_shelf_capacity.checked_sub(thickness) {
            current_shelf_capacity = reduced_capacity;
        }
        else {
            combined_shelf_height += current_shelf_height;

            current_shelf_capacity = shelf_width as u32 - thickness;
            current_shelf_height = 0;
        }
        current_shelf_height = i32::max(height, current_shelf_height);
    });

    combined_shelf_height + current_shelf_height
}