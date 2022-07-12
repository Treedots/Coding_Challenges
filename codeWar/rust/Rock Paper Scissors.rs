fn rps(p1: &str, p2: &str) -> &'static str  {
    if p1 == p2{
        return "Draw!"
    }
    //Comparing using Tuples
    match (p1,p2){
        ("scissors", "paper")|("paper","rock")|("rock","scissors") => return "Player 1 won!",
        _ => return "Player 2 won!"
    }
}