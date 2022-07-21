/*function score( dice ) {
  r = {};
  
  for(i in dice){
    const d = dice[i];
    if(r[d] == undefined)
      r[d] =1;
    else{
      r[d]++;
    }
  }

  s = 0;
  for(i=1;i<=6;i++){
    if(i==1){
      if(r[i]>=3){
        s += 1000;
        r[i] -= 3;
      }
      if(r[i]>0){
        s += r[i] * 100;
      }
    }
    else if(i==5){
      if(r[i]>=3){
        s += i * 100;
        r[i] -= 3;
      }
      if(r[i]>0){
        s += r[i] * 50;
      }
    }
    else{
      if(r[i]>=3){
        s += i * 100;
      }
    }
  }
  return s;
}*/

function score( dice ) {
  // Fill me in!
  dict = [0,0,0,0,0,0]
  dice.forEach((x)=>{dict[x-1]++;})
  sum = 0;
  dict.forEach((x,i)=>{
    sum += x >= 3 ? (i+1) * (i==0 ? 1000:100):0;
    x -= x >= 3 ? 3:0;
    sum += (i == 0 || i == 4) && x > 0 ? (i == 0 ? 100:50) * x:0;
  })
  return sum;
}
