Array.prototype.sameStructureAs = function (other) {
    // Return 'true' if and only if 'other' has the same
    // nesting structure as 'this'.

    // Note: You are given a function isArray(o) that returns
    // whether its argument is an array.
  
  //Check if Length is Correct.
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
};
