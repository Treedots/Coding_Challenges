fn set_alarm(employed: bool, vacation: bool) -> bool {
    match (employed,vacation){
        (true,false)=> true,
        _ => false
    }
}
