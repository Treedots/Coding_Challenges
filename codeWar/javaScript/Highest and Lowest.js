function highAndLow(numbers){
  //https://www.codewars.com/kata/554b4ac871d6813a03000035/solutions/javascript
  let a = numbers.split(" ");
  let max = a[0];
  let min = a[0];
  a.forEach((v)=>{
    min = Math.min(min,v);
    max = Math.max(max,v);
  })
  return `${max} ${min}`;
}
