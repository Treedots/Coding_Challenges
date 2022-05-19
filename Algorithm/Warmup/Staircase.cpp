void staircase(int n) {
    string s = "";
    for(int i=0;i<n;i++){
        s +=" ";
    }
    for(int i=0;i<n;i++){
        s = s.substr(1);
        s += "#";
        cout << s << endl;     
    }
}
