//First code on generics
fn main() {
    println!("Getting the largest in an array");

    let arr = [10, 10, 5, 6, 17, 39, 65];

    let largest = get_largest(&arr);

    println!("The largest number in the array is: {}", largest);
}

fn get_largest <T: PartialOrd> (arr: &[T]) -> &T {
    let mut large = &arr[0];

    for t in arr {
        if t > large {
            large = t;
        }
    }

    large
}