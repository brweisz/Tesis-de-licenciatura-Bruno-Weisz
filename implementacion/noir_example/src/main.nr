fn main(
    idx: Field,
    val: Field,
) -> pub Field {
    let mut arr: [Field; 10] = [0; 10];
    for _ in 0..15 {
        arr[idx] = val;
        assert(arr[idx] == val);
    }
    arr[idx]
}
