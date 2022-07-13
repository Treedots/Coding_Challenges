fn make_readable(seconds: u32) -> String {
    format!("{:0>2}:{:0>2}:{:0>2}",(seconds/60/60), (seconds/60) % 60,seconds % 60)
}
