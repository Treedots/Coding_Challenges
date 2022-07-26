function isSolved(board) {
  // TODO: Check if the board is solved!
  const pattern = (x) => {
    switch(x){
        case 0: return [[0,0],[0,1],[0,2]];
        case 1: return [[1,0],[1,1],[1,2]];
        case 2: return [[2,0],[2,1],[2,2]];
        case 3: return [[0,0],[1,0],[2,0]];
        case 4: return [[0,1],[1,1],[2,1]];
        case 5: return [[0,2],[1,2],[2,2]];
        case 6: return [[0,0],[1,1],[2,2]];
        case 7: return [[2,0],[1,1],[0,2]];
    }
  }
  contains_zero = false;
  
  for(i=0;i<8;i++){
    let ptt = pattern(i);
    let slot1 = board[ptt[0][0]][ptt[0][1]];
    let slot2 = board[ptt[1][0]][ptt[1][1]];
    let slot3 = board[ptt[2][0]][ptt[2][1]];
    
    if((slot1 == 0 || slot2 == 0 || slot3 == 0) && !contains_zero)
      contains_zero = true;
    else if(slot1==slot2&&slot2==slot3&&slot1!=0)
      return slot1;    
  }
  return contains_zero?-1:0;
}
