#include <cmath>
#include <cstdio>
#include <vector>
#include <iostream>
#include <algorithm>
using namespace std;



int main() {
    /* Enter your code here. Read input from STDIN. Print output to STDOUT */   
    class que{
        vector<int> data = {};
        public:
        void enqueue(int input){
            data.push_back(input);          
        }
        void dequeue(){
            if(data.size()>0){
                for(int i = 0;i<data.size()-1;i++){
                data[i] = data[i+1];
                }   
                data.pop_back();
            }            
        }
        void printfront(){
            if(data.size()>0){
                printf("%u\n",data[0]);
            }
        }
    };
    int q;
    cin >> q;
    que que01;
    for(int i=0;i<q;i++){
        int type;
        cin >> type;
        switch (type) {
            case 1:
                int input;
                cin >> input;
                que01.enqueue(input);
                break;
            case 2:
                que01.dequeue();
                break;
            case 3:
                que01.printfront();
                break;
        }
        //printf("%u %u \n",type,x);
    }
    
    
    return 0;
}
