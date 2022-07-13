fn enough(cap: i32, on: i32, wait: i32) -> i32 {
    let x = -(cap - on - wait);
    if x < 0{
        return 0;
    }
    return x;
}
