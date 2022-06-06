void miniMaxSum(vector<int> arr) {
  sort(arr.begin(),arr.end());
  unsigned long result = 0;
  for(int i:arr){
      result += i;
  }
  printf("%lu %lu",result-arr[arr.size()-1],result-arr[0]);
}
