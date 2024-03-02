fn get_larget_number<T: PartialOrd + Copy>(vec: Vec<T>) -> T {
    let mut largest = vec[0];

    for &x in &vec[1..] {
        if x > largest {
            largest = x
        }
    }

    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64, f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

fn main() {
    let numbers = vec![1000, 5, 25, 8, 15, 300];

    let float_nums = vec![1.3, 0.34, 0.47, 1.34, 2.4, 2.5];

    let largesti = get_larget_number(float_nums);
    let largestii = get_larget_number(numbers);

    println!("the largest number is :  {} and {} ", largesti, largestii);

    let p1 = Point { x: 30, y: 87 };
    let p2 = Point { x: 0.46, y: 8.2 };
    let p3 = Point { x: 6, y: 8.2 };
}
