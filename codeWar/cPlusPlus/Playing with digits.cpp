#include <cmath>
class DigPow
{
public:
  static int digPow(int n, int p){
    //convert to string
    std::string e = std::to_string(n);
    unsigned int r = 0;
    for(unsigned int i=0;i<e.size();i++){
      //convert text = int
      int a = std::stoi(e.substr(i,1));
      //add calculation into result
      r += std::pow(a,p);
      p++;
    }
    //if got remainer = -1
    if(r%n!=0) return -1;
    return r/n;
  }
};