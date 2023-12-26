fn main() {
    // conditions
    let is_rust_fun = true;
    if is_rust_fun {
        println!("Rust is fun");
    } else {
        println!("It is not fun")
    }
    let rate_rust = if is_rust_fun { 10 } else { 2 };

    println!("Rust rating is {rate_rust}");
    // expression
    let y = {
        let x = 20;
        x + 7
    };

    println!("the value of y is {y}");

    // looping
    let mut counter = 10;
    loop {
        println!("{counter}");
        counter -= 1;
        if counter == 0 {
            break;
        }
    }

    // while loop
    let mut num = 0;
    while num != 5 {
        println!("num is {num}");
        num += 1;
    }
    // for loop
    let a = [1, 5, 6, 3, 8, 9, 0, 6];

    for index in a {
        println!("the value is {index}")
    }
}
