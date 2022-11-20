use playground::factorial_recursive;

fn main() {
    for num in 0..=6 {
        let factorial = factorial_recursive(num);
        println!("Factorial of the number : {} is {}", num, factorial);
    }
}