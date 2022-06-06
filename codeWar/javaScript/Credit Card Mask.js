// return masked string
function maskify(cc) {
  if(cc.length>4){
    let x = cc.length-4;
    let c = "#".repeat(x);
    return c + cc.substring(x)
  }
  else{
    return cc;
  }
}