function duplicateCount(text){
  //...
  let r = {};
  let duplicates = 0;
  for(i=0;i<text.length;i++){
    let c = text[i].toLowerCase();
    if(r[c]==undefined) r[c] = 1;
    else{
      if(r[c]==1){
        duplicates++
      }
      r[c]++;
    }
  }
  return duplicates;
}