//First code on generics
fn main() {
    println!("Getting the largest in an array");

    let arr = [10, 10, 5, 6, 17, 39, 65];

    let largest = get_largest(&arr);

    println!("The largest number in the array is: {}", largest);
    
    let arr2 = ['a', 'f', 'g', 'w', 'v'];
    let largest2: Option<char>  = get_largest2(&arr2);

    match largest2 {
        None => println!("Array2 is empty"),
        Some(b) => println!("The largest char in the array2 is: {}", b)
    }
    
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

fn get_largest2 <B: PartialOrd + Copy> (arr: &[B]) -> Option<B> {
    if arr.is_empty() {
        return None;
    }
    let mut large = &arr[0];

    for b in arr {
        if b > large {
            large = b
        }
    }

    Some(*large)
}