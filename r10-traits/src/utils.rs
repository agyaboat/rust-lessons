
pub fn get_largest <T: PartialOrd> (arr: &[T]) -> &T {
    let mut large = &arr[0];

    for t in arr {
        if t > large {
            large = t;
        }
    }

    large
}

pub fn get_largest2 <B: PartialOrd + Copy> (arr: &[B]) -> Option<B> {
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