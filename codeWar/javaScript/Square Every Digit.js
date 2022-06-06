function squareDigits(num){
  let r = ""
  while(num>0){
    r = (num%10) * (num%10) + r;
    num = Math.floor(num/10);
  }
  return r==""?num:parseInt(r);
}