// The ownership concept

// fn take_ownership(str: String) {
//     println!("{} world", str);
// }

fn calculate_length(s: &String) -> usize {
    s.len()
}
fn main() {
    //name is not valid here
    {
        let name = "John";
        println!("{name}")
    } //name goes out of scope
      // println!("{name}");
    let x = 5;
    let mut y = x; //copy

    y += 8;

    println!("x = {x} and y = {y}");

    let s1: String = String::from("hello");
    // let s2 = s1;

    // println!("{},{}", s1, s2)
    // println!("{}", s2)
    // take_ownership(s1);

    // let len = calculate_length(s1);
    // println!("{} world ,len={}", s1, len)

    let len = calculate_length(&s1);
    println!("{} world ,len={}", s1, len);

    let sn = String::from("Hello");

    // let r1 = &sn;
    // let r2 = &mut sn;
    let r1 = &sn;
    let r2 = &sn;

    println!("{},{}", r1, r2);
}
