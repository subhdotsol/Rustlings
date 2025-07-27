fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec1 = vec.clone(); // clone vec0 to create a new Vec
    vec1.push(88); // modify the clone
    vec1 // return modified vec1
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(&vec0); // pass vec0 by reference

        assert_eq!(vec0, [22, 44, 66]); // vec0 untouched
        assert_eq!(vec1, [22, 44, 66, 88]); // vec1 modified copy
    }
}
