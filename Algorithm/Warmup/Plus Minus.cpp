void plusMinus(vector<int> arr) {
    double p=0;
    double n=0;
    double z=0;
    for(int a:arr){
        if(a>0){
            p++;
        }
        else if(a<0){
            n++;
        }
        else{
            z++;
        }
    }
    printf("%f\n%f\n%f",p/arr.size(),n/arr.size(),z/arr.size());
}
