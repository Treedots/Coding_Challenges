fn divisors(n: u32) -> u32 {
   let mut div:u32 = 0;
   for i in 1..=n {
        if n % i == 0 {
            div += 1;
       }
    }
    return div;
}
