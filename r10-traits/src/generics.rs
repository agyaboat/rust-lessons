use crate::utils;

pub fn main(){
     // GENERICS HERE !!!!
    println!("Getting the largest in an array");
    let arr = [10, 10, 5, 6, 17, 39, 65];
    let largest = utils::get_largest(&arr);

    println!("The largest number in the array is: {}", largest);
    let arr2 = ['a', 'f', 'g', 'w', 'v'];
    let largest2: Option<char> = utils::get_largest2(&arr2);
    match largest2 {
        None => println!("Array2 is empty"),
        Some(b) => println!("The largest char in the array2 is: {}", b)
    }
}