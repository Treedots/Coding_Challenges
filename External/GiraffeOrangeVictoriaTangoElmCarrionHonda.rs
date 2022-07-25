use std::collections::HashMap;
use std::cmp;
/*
  Simple Reimplementation of a Coding Challenges into Rust.
  https://replit.com/@treedotsu/GiraffeOrangeVictoriaTangoElmCarrionHonda
*/
fn main() {
  // Run Challenge 01:
  println!("{:?}",code_challenge_01(3,3,vec![vec![50,40,70],vec![60,80,90],vec![20,40,80]]));
  // Run Challenge 02:
  println!("{}",code_challenge_02("","ascata1tcfsh1231"));

  
}
/*
  Challenge 01
  3 Inputs
  Input m - Number of Students
  Input n - Number of Subjects
  Input o - Array of Results

  Remove Subejct with the lowest Average among the Students And return the total score per Student.
*/
fn code_challenge_01(m:i32,n:i32,o:Vec<Vec<i32>>) -> Vec<i32>{
  
  let mut result: Vec<i32> = vec![0;m as usize];
  let mut lowest:i32 = -1;
  let mut lowest_score:i32 = m * 100 +1;
  for x in 0..n{
    let mut sum = 0;
    for y in 0..m{
      result[y as usize] += o[y as usize][x as usize];
      sum += o[y as usize][x as usize]; 
    }
    //Check Lowest Score
    if lowest_score > sum{
      lowest_score = sum;
      lowest = x;
    }
  }
  for y in 0..m{
    result[y as usize] -= o[y as usize][lowest as usize];
  }
  return result;
}

/*Challenge 02
  Two Inputs
  Input n - String e.g cat
  Input m - List of Random String to Generate From
  e.g kdai123j1LJFlajdlcat

  Output String Structure
  
  n * no of times generated from m and remaining chars not used in sort lowercase , uppercase, digits.
  
  After each n text seperate with -
  [n]-[n]-[lower][upper][digit]

*/

fn code_challenge_02(n:&str,m:&str) -> String{
  //If no chars to get from return empty;
  if m.is_empty(){
    return "".into();
  }
  let mut char_map: HashMap<char,i32> = HashMap::new();
  let mut char_list: Vec<char> = vec![];
  for ch in m.chars(){
    // Entry(ch)
    
      let count = char_map.entry(ch).or_insert(0);
      if *count == 0{
        char_list.push(ch);
      }
      *count += 1;
    
  }
  let mut text_list: Vec<char> = vec![];
  let mut text_map: HashMap<char,i32> = HashMap::new();
  let mut min_repeats = -1;
  if !n.is_empty(){
    for ch in n.chars(){
      let count = text_map.entry(ch).or_insert(0);
      if *count == 0{
        text_list.push(ch);
      }
      *count += 1;
       
  }
  for ch in text_list{
    if min_repeats == -1{
      min_repeats = char_map[&ch]/text_map[&ch];
    }
    else{
      min_repeats = cmp::min(min_repeats,char_map[&ch]/text_map[&ch]);
    }
  }
  }
  
  
  //println!("{:?}",char_map);
  //println!("{:?}",text_map);
  
  char_list.sort_by(|a, b| a.cmp(b));
  
  let amended_text = format!("{}-",n);
  let front = if min_repeats >0{
    amended_text.repeat(min_repeats as usize)
  }
    else{
      "".into()
    };
  
  let mut lower_block:Vec<String> = Vec::new();
  let mut upper_block:Vec<String> = Vec::new();
  let mut digit_block:Vec<String> = Vec::new();
  for ch in char_list{
    let number_of_remaining:i32= 
      if text_map.contains_key(&ch){
        char_map[&ch] - text_map[&ch] * min_repeats
      }
      else{
        char_map[&ch]
      };
    let rep_text = ch.to_string().repeat(number_of_remaining as usize);
    match ch {
       '0'..='9' => digit_block.push(rep_text),
      'a'..='z' => lower_block.push(rep_text),
      'A'..='Z' => upper_block.push(rep_text),
      _ => panic!("Invalid Char")
    }
  }
  
  return format!("{}{}{}{} ",front,lower_block.concat(),upper_block.concat(),digit_block.concat());
  
}
