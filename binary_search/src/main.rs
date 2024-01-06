fn binar_serch(vec: &Vec<i32>, target: i32) -> Option<usize> {
    let mut first = 0;
    let mut last = vec.len() - 1;
    while first <= last {
        let mid = (first + last) / 2;
        if vec[mid] == target {
            return Some(mid);
        } else if vec[mid] < target {
            first = mid + 1;
        } else {
            last = mid - 1;
        }
    }
    None
}

fn main() {
    let numbers = vec![1, 3, 5, 8, 9, 20, 18];
    let target = binar_serch(&numbers, 20);
    println!("{:?}", target);
}
