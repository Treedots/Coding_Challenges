//https://www.codewars.com/kata/546f922b54af40e1e90001da/train/rust
fn alphabet_position(text: &str) -> String {
    // Code here...
    let mut result = "".to_owned();
    for i in text.chars(){
        let j = i.to_digit(36);
        if j == None{
            continue
        }
        let t = j.unwrap();
        if t >= 10 && t<=36{
            result = format!("{} {}",result,t-9);
        }
    }
    result.trim().to_string()
}
