fn process_numbers(numbers: &[i32]) {
    let mut sum = 0;
    for number in numbers {
        sum += number;
    }

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

fn main() {
    process_numbers(&[1, 7, 4, 9, 2, 3]);
    let chunk = split_string("hello,world".to_string(), ',', 1);
    println!("Split string : {chunk}")
}
