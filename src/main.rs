use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
    println!("Three hours in seconds is: {}", THREE_HOURS_IN_SECONDS);

    let y = 5;
    let y = y +1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }
    println!("The value of y is: {}", y);

    //array initial value
    let _a = [3; 5]; //3 is the initial value, 5 is how many elements are in the array

    let b = [1, 2, 3, 4, 5];
    println!("Please enter an array index:");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = b[index];

    println!("The value of the element at index {} is: {}", index, element);

}
