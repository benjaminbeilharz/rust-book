use std::io;

pub fn solve() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = {
        let c = 4;
        c + 1
    };

    println!("The value of b is {}", b);
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {} is {}", index, element);
    
}
