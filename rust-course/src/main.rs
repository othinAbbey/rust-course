#![deny(clippy::all)]

// use std::ffi::c_float;

//Variables
fn main() {
    // Declare a mutable variable 'likes' and initialize it with "Food".
    let mut likes: &str = "Food";
    let data: &str = "Ten";

    // Declare immutable variables 'last_name' and 'much'.
    let last_name: &str = "John";
    let much: &str = "a lot";

    // Shadow the 'likes' variable with a new value.
    likes = "pork";

    //we can also change the data type using data mutations
    let data: i32 = 69;

    //declare a signed varibale thats values both negative and poistive
    let _age: i8 = -20;

    //declrae an unsighed varibale that takes numbers from 0 to 20 only poistives
    let _age: u8 = 120;

    //floating points are declared with a float. thes are numbers with decimals
    let _temp: f32 = 32.5;

    // Print the values of 'last_name', 'likes', and 'much'.
    println!(
        "{} loves {} this much {} and data has been changed from string {}",
        last_name, likes, much, data,
    );

    println!("Data changed from {} to {} due to shadowing", data, data);
}
