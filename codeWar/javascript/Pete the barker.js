//https://www.codewars.com/kata/525c65e51bf619685c000059/solutions/javascript
function cakes(recipe, available) {
  let lowestValue = 100;
  Object.entries(recipe).forEach(([k,v])=>{
    let value_avaliable = available[k];
    if(value_avaliable==undefined){
      lowestValue = 0;
    }
    let r =  value_avaliable / v;
    if(lowestValue>r){
      lowestValue = Math.floor(r);
    }
  })
  return lowestValue;
}
