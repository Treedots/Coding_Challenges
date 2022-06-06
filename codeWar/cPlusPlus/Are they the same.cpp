#include <vector>

class Same {
public :
    static bool comp(std::vector<int>&a, std::vector<int>&b) {
      // your code
      std::map<int,int> m;
      for(int i:a) m[i*i]++;
      
      sort(b.begin(),b.end());
      for(int i:b){
        int j = m[i];
        if(j==0) return false;
        m[i]--;
      }
      return true;
    }
};