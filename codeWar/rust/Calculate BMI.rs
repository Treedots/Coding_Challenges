fn bmi(weight: u32, height: f32) -> &'static str {
  let bmindex = weight as f32/(height*height);
  match bmindex{
      x if x <= 18.5 => "Underweight",
      x if x <= 25.0 => "Normal",
      x if x <= 30.0 => "Overweight",
      _ => "Obese"
    }
}