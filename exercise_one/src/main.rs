/*
 Print the lyrics to the Christmas carol
  “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
*/
fn main() {
    let twelve_day_of_christmas = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    ];
    for (index, &value) in twelve_day_of_christmas.iter().enumerate() {
        println!("On the {index} day of Christmas my true love said to me {value}")
    }
}
