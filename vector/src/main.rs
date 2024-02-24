// vectors

fn get_vec_item(index: usize, vec: &Vec<i32>) -> Option<&i32> {
    if index > vec.len() {
        return None;
    }

    let value: Option<&i32> = vec.get(index);
    value
}
// fn get_vec_item(index: usize, vec: &Vec<i32>) -> Option<&i32> {
//     vec.get(index)
// }
fn main() {
    // let mut prices = vec![20, 40, 50, 57];

    // for prc in &prices {
    //     println!("prices = {}", prc);
    // }

    // let last_item = prices.last().unwrap();
    // println!("{}", last_item)

    let mut prices = vec![20, 40, 50, 57];

    // let item_2 = get_vec_item(2, &prices);

    // println!("{:?}", item_2)

    // adding item to a vector

    prices.push(4);

    println!("{:?}", prices);

    let mut more_prices = vec![9, 24, 7];
    // extends add each element of the given slice to the vector
    // prices.extend(more_prices);

    // append adds the given vector to the vector , requires the vector to be mutable
    prices.append(&mut more_prices);

    // insert item at a given index
    prices.insert(0, 2);
    println!("{:?}", prices);
}
