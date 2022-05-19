int diagonalDifference(vector<vector<int>> arr) {
    int result_a = 0;
    int result_b = 0;
    for(int i=0;i<arr.size();i++){
        result_a += arr[i][i];
        result_b += arr[arr.size()-1-i][i];
    }
    return abs(result_b-result_a);
}
