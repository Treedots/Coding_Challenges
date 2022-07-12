fn digitize(n: u64) -> Vec<u8> {
    // your code here
    let mut result: Vec<u8> = Vec::new();
    
    for a in n.to_string().chars().rev(){
        result.push(a as u8 - 48)
    }
    result
}