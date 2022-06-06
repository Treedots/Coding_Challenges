int birthdayCakeCandles(vector<int> candles) {
    sort(candles.begin(),candles.end());
    int result = 1;
    int current = candles[candles.size()-1];
    for(int i=candles.size()-2;i>=0;i--){
        if(candles[i]==current){
            ++result;
        }
        else{
            break;
        }
    }
    return  result;
}
