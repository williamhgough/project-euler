fn main() {
    println!("Solution 1: {:#?}", solution3())
}

fn solution1() -> i32 {
    let mut sum = 0;
    for num in 1..1000 {
        if num % 3 == 0 || num % 5 == 0 {
            sum += num
        }
    }
    sum
}

fn solution2() -> i32 {
    let numbers = 0..1000;
    numbers
        .filter(|n| n % 3 == 0 || n % 5 == 0)
        .collect::<Vec<i32>>()
        .iter()
        .fold(0, |acc, &x| acc + x)
}

fn solution3() -> i32 {
    (0..1000)
        .filter(|n| n % 3 == 0 || n % 5 == 0)
        .fold(0, |acc, x| acc + x)
}
