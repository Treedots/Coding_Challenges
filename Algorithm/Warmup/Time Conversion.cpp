string timeConversion(string s) {
    string ap = s.substr(8);
    string mid = s.substr(2,6);
    string frt = s.substr(0,2);
    if(frt=="12"&&ap=="AM"){
        frt = "00";
    }
    else if(ap=="PM"&&frt!="12"){
        int i = stoi(frt)+12;
        frt = to_string(i);
    }
    
    return frt+mid;
}
