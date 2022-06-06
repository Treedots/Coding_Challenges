#include <cmath>
#include <cstdio>
#include <vector>
#include <iostream>
#include <algorithm>
using namespace std;


int main() {
    /* Enter your code here. Read input from STDIN. Print output to STDOUT */
    class texteditor{
        string s = "";
        
        vector<string> undo_list;
        public:
        void append(string input){
            undo_list.push_back(s);
            s += input;
        }
        
        void delete_s(int i){
            undo_list.push_back(s);
            s =  s.substr(0,s.length()-i);
        }
        
        void print_s(int i){
            cout << s[i-1] << endl;
        }
        
        void undo_s(){
            s = undo_list.back();
            undo_list.pop_back();
            //cout << "undo!!"<<endl;
        }
    };
    
    texteditor t;
    
    int o;
    cin >> o;
    
    cin.ignore();
    string s = "";
    for(int i=0;i<o;i++){
        getline(cin,s);
        switch (s[0]) {
        case '1':{
            t.append(s.substr(2));
            break;
        }
        case '2':{
            int j = stoi(s.substr(2));
            t.delete_s(j);
            break;
        }
        case '3':{
            int j = stoi(s.substr(2));
            t.print_s(j);
            break;
        }
        case '4':{
            t.undo_s();
            break;
        }
        }
        
    }
    
    return 0;
}
