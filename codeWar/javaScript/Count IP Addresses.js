function ipsBetween(start, end){
  //https://www.codewars.com/kata/526989a41034285187000de4/javascript
  const index_ref = [256*256*256,256*256,256,1];
  start_value = 0;
  start.split('.').map((v,i)=>(start_value += parseInt(v)*index_ref[i]));
  end_value = 0;
  end.split('.').map((v,i)=>(end_value += parseInt(v)*index_ref[i]));
  //console.log(start_value,end_value,end_value-start_value);
  return end_value-start_value;
}
