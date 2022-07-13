fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty(){
        [].to_vec()
    }
    else{
        let mut a: Vec<i32> = [0,0].to_vec();
    for i in input{
        if i > 0 {
            a[0] += 1;
        }
        else{
            a[1] += i;
        }
    }
    a
    }
    
}
