int superDigit(string n, int k) {
    long result = 0;
    if(n.length()==1&&k==1) return n[0]-48;
    for(int i = 0;i<n.length();i++){
        result += n[i]-48;
    }
    result *= k;
    return superDigit(to_string(result), 1);
}
