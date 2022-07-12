fn first_non_consecutive(arr: &Vec<i32>) -> Option<i32> {
  // code here
    let mut lastNo:i32 = arr[0];
    for i in arr{
        if i - lastNo > 1{
            return Some(*i);
        }
        lastNo = *i;
    }
    return None;
}