string gridChallenge(vector<string> grid) {
  //Sort all Grid
  for(string & s : grid){
      sort(s.begin(),s.end());
  }
  //sort(grid.begin(),grid.end());
  int gridlength = grid[0].length();
  for(int x = 0;x<gridlength;x++){
      string s = "";
      for(int y=0;y<grid.size();y++){
       s += grid[y][x];  
      }
      string c = s;
      sort(c.begin(),c.end());
      if(c!=s) return "NO";
  } 
  return "YES";
}
