fn anagrams(word: &str, words: &[String]) -> Vec<String> {
    // https://www.codewars.com/kata/523a86aa4230ebb5420001e1/rust
    let mut result: Vec<String> = Vec::new();
    let mut j:Vec<char> = word.chars().collect();
    let word_length = word.len();
    j.sort_by(|a,b| b.cmp(a));
    for i in words{
        if word_length == i.len(){
            let mut w:Vec<char> = i.chars().collect();
            w.sort_by(|a,b| b.cmp(a));
            if w == j{
                result.push(i.to_string());
            }
        }
        
    }
    result
}
