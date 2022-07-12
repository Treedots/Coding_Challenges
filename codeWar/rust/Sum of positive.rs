fn positive_sum(slice: &[i32]) -> i32 {
    // your code
    let mut a = 0;
    for i in slice{
        if i > &0{
            a += i;
        }
    }
    return a
}