//https://www.codewars.com/kata/546f922b54af40e1e90001da/solutions/javascript
function alphabetPosition(text) {
  const alpha = "abcdefghijklmnopqrstuvwxyz";
  let t = text.split("");
  let result = "";
  t.forEach((v)=>{
    let a = v.toLowerCase();
    if (!/[a-z]/.test(a)){
      return 
    }
    let p = alpha.match(a);
    result += p.index + 1+" ";
  })
  return result.trim();
}
