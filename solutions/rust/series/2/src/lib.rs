pub fn series(digits: &str, len: usize) -> Vec<String> {
    /*
    // Attempt 1
    if len == 0 || len > digits.len() {
        return Vec::new();
    }
    
    let mut result = Vec::new();
    for starting_index in 0..=(digits.len() - len) {
        let substring = &digits[starting_index..starting_index + len];
        result.push(substring.to_string());
    }
    result
    */

    // Attempt 2: AI generated solution to Return a vector containing as many empty strings as the length of the string "digits" plus one.
    // Prompt: Exercism Instructions + "what Rust program would you write that has the natural return function of vector of empty strings for input of n=0?"
    let length = digits.len();

    // Error case: If len is greater than the total length, return empty.
    // We handle this first to avoid subtracting usize and getting an underflow.
    if len > length {
        return Vec::new();
    }

    // The number of possible starting positions for a substring of length len
    // in a string of length L is L - len + 1.
    let num_series = length - len + 1;

    // Iterate through all possible starting indices (i).
    // This loop works for both cases:
    // 1. If len > 0, the range is 0..(L - len + 1).
    // 2. If len = 0, the range is 0..(L - 0 + 1), or 0..(L + 1).
    //    This gives L+1 iterations, one for each starting boundary.
    (0..num_series)
        .map(|i| {
            // Slice the string: S[i .. i + len]
            // 1. If len > 0: This extracts a substring of length len.
            // 2. If len = 0: This extracts S[i .. i + 0], which is S[i .. i].
            //    In Rust, slicing where the start and end indices are the same
            //    results in an **empty string ("")**.
            digits.get(i..i + len)
                // The .unwrap_or_default() is defensive, but mathematically
                // i+len is guaranteed to be <= length due to the num_series calculation.
                .unwrap_or_default()
                .to_string()
        })
        .collect()
}

/*
Different languages on Exercism have different expectations about what the result should be if the length of the substrings is zero. On the Rust track, we don't have a test for that case, so you are free to do what you feel is most appropriate.

Consider the advantages and disadvantages of the following possibilities:

Crash the program with panic!. // if the function was used in a larger piece of code, this would crash the entire program - not very friendly.
Return a Result::Err. (not possible here, because the function signature is given)
Return an empty vector. // this is the natural return type for an implementation that iterates over the digits (my implementation does this).
Return a vector containing as many empty strings as the length of the string "digits" plus one. (this has some nice mathematical properties!)
 */