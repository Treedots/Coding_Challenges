int josephusSurvivor(int n, int k) {
  // your code here
  if(n==1) return 1;
  else{
    return (josephusSurvivor(n - 1, k) + k - 1) % n + 1;
    
    
  }
}