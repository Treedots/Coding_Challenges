decodeMorse = function(morseCode){
  r = morseCode.split(" ").map((v)=>MORSE_CODE[v]||' ').join('').replace(/\s{2}/g," ").trim();
  return r;
}
