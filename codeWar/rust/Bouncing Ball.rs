fn bouncing_ball(h: f64,  bounce: f64,  window: f64) -> i32 {
    // Check conditions are met or else return -1
    if h  <= 0.0 || bounce <= 0.0 || bounce >=1.0 || window >= h{
        -1
    }
    else{
        //As ball dropping down, ball will pass the window once
        let mut seen: i32 = 1;
        //Calculate the height of the bounce
        let mut current_height: f64 = h * bounce;
        // Loop until the ball cant be seen and return the no of bounces
        while current_height > window{
            seen += 2;        
            current_height = current_height * bounce;
        }
        seen
    }
    
}