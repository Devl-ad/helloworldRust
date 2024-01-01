use std::io;

fn process_numbers(sum: i32) {
    println!("Sum = {sum}");

    if sum % 2 == 0 {
        println!("sum is even")
    } else {
        println!("Some is odd")
    }
}

fn split_string(s: String, delimeter: char, field: usize) -> String {
    let part: Vec<&str> = s.split(delimeter).collect();
    let result = part.get(field);
    result.expect("OPPs something went wrong").to_string()
}

fn sum_number(nums: &[i32]) -> i32 {
    let mut sum = 0;
    for num in nums {
        sum += num
    }
    sum
}

fn fib(n_index: i32) -> i32 {
    if n_index <= 1 {
        n_index
    } else {
        fib(n_index - 1) + fib(n_index - 2)
    }
}

fn main() {
    let sum = sum_number(&[1, 7, 4, 9, 2, 3]);
    process_numbers(sum);
    let chunk = split_string("hello,world".to_string(), ',', 1);
    println!("Split string : {chunk}");

    println!("Finding the fib for an index");
    println!("Input the fib index you want to find");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to readline");
    let fib_index = input.trim().parse().expect("Please enter a number");
    let result = fib(fib_index);
    println!("Fib of {fib_index} is = {result}")
}
