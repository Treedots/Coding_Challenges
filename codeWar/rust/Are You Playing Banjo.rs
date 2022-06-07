fn are_you_playing_banjo(name: &str) -> String {
    let c =  name.chars().nth(0).unwrap();
    match c {
        'r'|'R' => String::from(name.to_owned() + " plays banjo"),
        _ => String::from(name.to_owned() + " does not play banjo")
    }
}