fn main(
    idx: Field,
    val: Field,
) -> pub Field {
    let mut arr: [Field; 100] = [0; 100];
    for _ in 0..N {
        arr[idx] = val;
        assert(arr[idx] == val);
    }
    arr[idx]
}
