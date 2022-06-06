string isBalanced(string s) {
    string seq = "{}()[]";
    vector<int> queue;
    for(int i = 0;i<s.length();i++){
        int e = (seq.find(s[i])+1)%2?-1:1;
        int f = e * (seq.find(s[i])+1)/2;
        printf("%d,%d,%d",i,e,f);
        if(e>0){
            if(queue.size()==0){
                return "NO";
            }
            else if(abs(f)!=abs(queue.back())){
                return "NO";
            }
            else{
                queue.pop_back();
            }
        }
        else{
            queue.push_back(f);
        }
    }
    return queue.size()>0?"NO":"YES";
}
