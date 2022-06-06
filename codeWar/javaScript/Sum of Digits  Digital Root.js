function digital_root(n) {
  // ...
  let r = 0;
  if(n<10) return n;
  
  while(n>0){
    r += n % 10;
    n = Math.floor(n/10);
  }
  //console.log(r);
  if(r>9){
    r = digital_root(r);
  }
  return r;
  
}