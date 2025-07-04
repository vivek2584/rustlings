fn main() {
    // You can optionally experiment here.
    let a = [1, 2, 3, 4, 5];
    let mut slice: [i32; 3] = [0; 3];
    slice.clone_from_slice(&a[1..4]);
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // let nice_slice = ???
        let nice_slice: [i32; 3] = a[1..4].try_into().unwrap();
        assert_eq!([2, 3, 4], nice_slice);
    }
}
