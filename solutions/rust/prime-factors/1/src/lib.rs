pub fn factors(n: u64) -> Vec<u64> {
    let mut result = vec![];
    // We have to add a base case, as our loop
    // for i in 2.. {
    //     if n % i == 0 {
    // would run forever if n <= 1.
    if n <= 1 {
        return result;
    }
    // we don't have to explicitly iterate over primes, because we would check if the lower numbers in our iteration divides our number and its factors cleanly.
    // the recursion will divide n by all its factors of 2, leaving no factor of 4 when it comes to that iteration.
    // so in effect, this loop does iterate over primes.
    for i in 2.. {
        if n % i == 0 {
            result.push(i); // i is a factor of n
            result.extend(factors(n / i)); // extend results recursively
            // Let's say n = 60, i = 2. We have pushed 2 to result, then called factors(30).
            // There is no need to iterate further to i = 3, 4, etc. because the recursive call will have already handled that.
            // So we can end early.
            return result;
        }
    }
    result
}
