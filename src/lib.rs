pub fn nth(n: u32) -> u32 {
    const UPPER_BOUND: u32 = 1000003;
    let mut integers = Vec::new();
    let mut count: u32 = 3;
    while count <= UPPER_BOUND {
        integers.push(count);
        count = count + 1
    }
    let mut prime_numbers: Vec<u32> = Vec::new();
    prime_numbers.push(2);
    for num in integers {
        let test_range = (num as f32).sqrt() as u32;
        for prime in &prime_numbers {
            let test = *prime;
            if test <= test_range {
                if num % prime == 0 {
                    break;
                }
            } else if test > test_range {
                prime_numbers.push(num);
                break;
            }
        }
    }
    prime_numbers[n as usize]
}
