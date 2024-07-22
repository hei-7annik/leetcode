/// Construct reversed_number from number - digit by digit
///
/// Edge-Cases: min/max i32 values, number that, reversed are lager/smaller then max/min i32...
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