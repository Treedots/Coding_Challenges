function rgb(r, g, b){
  // complete this function  
  let r1 = r>=0?(r<=255?r:255):0;
  let g1 = g>=0?(g<=255?g:255):0;
  let b1 = b>=0?(b<=255?b:255):0;
  
  return convertHex(r1)+convertHex(g1)+convertHex(b1);
}

function convertHex(a){
  let d = Math.floor(a%16);
  let c = Math.floor(a/16);
  let r = ""
  if(c<=9) r+= c.toString();
  else{r+=String.fromCharCode(55+c);}
  if(d<=9) r+= d.toString();
  else{r+=String.fromCharCode(55+d);}
  return r;
  
}