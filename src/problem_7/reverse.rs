#[inline]
pub const fn more_then_one_digit_long(number: i32) -> bool {
    number >= 10 || number.is_negative() && number <= -10
}

/// constructs **reversed_number** `rn` from a signed decimal **number** `n`
/// where the last digit of `n` is the first digit of `rn` and so forth.
///
/// # Example:
/// ```rust
/// assert_eq!(reverse(67234), 43276);
/// ```
///
/// # Cases:
/// 1. `n` is positive (including 0)
/// 2. `n` is negative
/// 3. `n` has trailing zero's
/// 4. Reversing `n` results in `rn` > i32::MAX or `rn` < i32::MIN
///
/// # Method:
/// 1. Move last digit from **number** `n` to **reversed_number** `rn` using modulo and addition
///
///    `n = 1234`, `rn = 0` -> `n = 1230`, `rn = 4`
///
/// 2. Shift digits of **number**/**reversed_number** one to the right/left using division
///
///     `n = 1230`, `rn = 4` -> `n = 123`, `rn = 40`
///
/// 3. Repeat until `n = 0`
///
/// The operations used preserve the sign
///
pub fn reverse(mut number: i32) -> i32 {
    let mut reversed_number = 0;

    while more_then_one_digit_long(number) {
        reversed_number += number % 10;
        number /= 10;

        match reversed_number.checked_mul(10) {
            Some(res) => reversed_number = res,
            None => return 0,
        }
    }

    reversed_number.checked_add(number).unwrap_or_default()
}