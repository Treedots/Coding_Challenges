//https://www.codewars.com/kata/52774a314c2333f0a7000688/javascript
function validParentheses(parens) {
  // your code here ..
  let len = parens.length;
  if( len % 2 == 1)
      return false;
  else{
    let queue = [];
    for(i=0;i<len;i++){
      let t = parens[i];
      if(queue.length == 0 && t == ")"){
        return false
      }
      else if(queue[queue.length-1]=="(" && t == ")"){
        queue.shift();
      }
      else{
        queue.push(parens[i]);
      }
    }
    return queue.length>0?false:true;
  }
}
