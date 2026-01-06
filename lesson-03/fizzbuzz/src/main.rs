fn fizzbuzz(n: i32) -> String {
    if n % 15 == 0 {
        String::from("FizzBuzz")
    } else if n % 3 == 0 {
        String::from("Fizz")
    } else if n % 5 == 0 {
        String::from("Buzz")
    } else {
        n.to_string()
    }
}

fn main() {
    println!("=== JEU FIZZBUZZ ===");

    for i in 1..=100 {
        println!("{}: {}", i, fizzbuzz(i));
    }
}
