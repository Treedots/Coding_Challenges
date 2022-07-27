//https://www.codewars.com/kata/526989a41034285187000de4/rust
fn ips_between(start: &str, end: &str) -> u32 {
    return proc(end)-proc(start);
}
fn proc(s:&str)->u32{
    return s.split(".").map(|v|{v.parse::<u32>().unwrap()}).reduce(|a,b|{a*256+b}).unwrap();
}
