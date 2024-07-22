/// Construct reversed_number from number - digit by digit
///
/// # input: i32 from -2.147.483.648 up to 2.147.483.647
///
/// # edge-cases:
///     - No Change             0                 -> 0
///     - leading zero's        1000              -> 1
///     - Result > i32::MAX     8.463.847.421     -> 0
///     - Result < i32::MIN     -2.147.483.647    -> 0
///
/// # Example  reverse(16234) = 43261
///

pub fn reverse(x: i32) -> i32 {
    let mut number = x;

    let mut reversed_number = 0;

    while number >= 10 || number.is_negative() && number <= -10 {
        reversed_number += number % 10;
        number /= 10;

        match reversed_number.checked_mul(10) {
            Some(res) => reversed_number = res,
            None => return 0,
        }
    }

    reversed_number.checked_add(number).unwrap_or_default()
}