// Sring

fn main() {
    // string slice the size is know at compile time so it is allocated to the stack

    let sentence = "hi i am sammy";

    println!("{}", &sentence[0..=4]);

    //concatinating string using format

    let desc = format!("Greateings :  {}", sentence);

    println!("{desc}");

    //iterating over a character in a sentence
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel : {}", c),
            _ => continue,
        }
    }

    // String it size is not known at compile time and it allocated to the heap, string can be mutable and flexible
    let name = String::from("Hello");
    println!("{}", name);

    // split and collect into a vector
    let word: Vec<&str> = sentence.split_whitespace().collect();
    println!("{:?}", word)
}
