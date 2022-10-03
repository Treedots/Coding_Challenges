//https://www.codewars.com/kata/559590633066759614000063/solutions/rust
use std::cmp;
fn min_max(lst: &[i32]) -> (i32, i32) {
    let mut min = lst[0];
    let mut max = lst[0];
    for i in lst{
      min =  cmp::min(min,*i);
      max = cmp::max(max,*i);
    }  
    
    (min,max)
}
