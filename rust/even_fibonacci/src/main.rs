fn main() {
    let sum: u32 = fibonacci_sequence()
        .filter(|&v| v % 2 == 0)
        .take_while(|&v| v <= 4000000u32)
        .sum();
    println!("Result: {}", sum);
}

pub struct FibonacciSequence {
    n_minus_1: u32,
    n_minus_2: u32,
}

impl Iterator for FibonacciSequence {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let mut result = self.n_minus_1 + self.n_minus_2;

        if result == 0 {
            result = 1;
        }
        self.n_minus_2 = self.n_minus_1;
        self.n_minus_1 = result;
        Some(result)
    }
}

pub fn fibonacci_sequence() -> FibonacciSequence {
    FibonacciSequence {
        n_minus_1: 0,
        n_minus_2: 0,
    }
}
