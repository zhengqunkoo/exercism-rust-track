pub fn is_armstrong_number(num: u32) -> bool {
    // convert num to string
    let num_str = num.to_string();
    let num_digits = num_str.len() as u32;
    let mut sum = 0;
    for digit in num_str.chars() {
        let digit_value = digit.to_digit(10).unwrap();
        // raise digit to the power of num_digits and add to sum
        sum += digit_value.pow(num_digits);
    }
    sum == num
}
