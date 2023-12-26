fn main() {
    println!("Hello, world!");
    // Variable
    let i: i32; // initializin a variable with value
    i = 30;

    println!("I is = {i}");
    let name = "sammy";
    println!("my name is {}", name);

    let mut x = 18;

    println!("{}", x);

    x = 3 + 10;

    println!("{}", x);
    // variable shadowing
    let y = 20;

    let y = y + 1;
    {
        // scoped
        let y = y + 17;
        println!("scoped y is =  {}", y)
    }

    println!("{}", y);

    // Datatypes

    let my_float = 0.67;

    println!("my float {my_float}");

    let remainder = 40 % 7;
    println!("remainder = {remainder}");

    let is_rust_fun = true;
    println!("Is rust fun {is_rust_fun}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("c = {c} , z = {z} , cat  = {heart_eyed_cat} ");

    //  Tuples

    let tup = (30, 0.7, "sammy");
    let (_numi, numii, _numiiii) = tup;

    println!("The value of numii is = {numii}");

    let q: (i32, i32, i32) = (2, 4, 7);
    println!("q index o is {}", q.0);

    // Arrays
    let a = [1, 4, 2, 7, 8, 6];
    println!(" array of index 3 is {}", a[3]);
}
