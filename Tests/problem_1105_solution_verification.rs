#[cfg(test)]
mod problem_1105 {
    use leetcode::problem_1105::min_height_shelves::{min_height_shelves};
    mod solution {
        use super::*;
        #[test]
        fn with_seven_books_and_two_thicknesses() {
            let books = [[1,1],[2,3],[2,3],[1,1],[1,1],[1,1],[1,2]]
                .iter()
                .map(|book| book.to_vec())
                .collect();
            assert_eq!(min_height_shelves(books, 4), 6);
        }

        #[test]
        fn with_three_books_and_three_thicknesses() {
            let books = [[1,3],[2,4],[3,2]]
                .iter()
                .map(|book| book.to_vec())
                .collect();
            assert_eq!(min_height_shelves(books, 6), 4);
        }

        #[test]
        fn with_two_narrow_tall_and_two_wide_and_short_books() {
            let books = [[7,3],[8,7],[2,7],[2,5]]
                .iter()
                .map(|book| book.to_vec())
                .collect();
            assert_eq!(min_height_shelves(books, 10), 15);
        }

        #[test]
        fn with_a_square_book() {
            let books = [[9,9],[5,4],[3,1],[1,5],[7,3]]
                .iter()
                .map(|book| book.to_vec())
                .collect();
            assert_eq!(min_height_shelves(books, 10), 17);
        }
    }
}