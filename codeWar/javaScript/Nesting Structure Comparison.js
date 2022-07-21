/*Array.prototype.sameStructureAs = function (other) {
  if (this.length != other.length)
    return false;
  
  for(i=0;i<this.length;i++){
    if(isArray(this[i])!=isArray(other[i]))
               return false;
    else if(isArray(this[i])){
      if (this[i].length != other[i].length)
               return false;
      else if(isArray(this[i][0])!=isArray(other[i][0])){
        return false;
      }
    }
  }
  return true
};*/
Array.prototype.sameStructureAs = function (other) {

  return (this.length === other.length) ? this.every((a,i) =>{
    // if Array do Recursive;
    return isArray(a) ? a.sameStructureAs(other[i]): true
  }) : false;
};

