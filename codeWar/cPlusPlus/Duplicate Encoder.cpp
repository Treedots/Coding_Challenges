#include <string>
std::string duplicate_encoder(const std::string& word){
  std::map<char,int> m;
  for(char c : word) m[std::tolower(c)]++;
  
  std::string result;
  for(char c: word) result += m[std::tolower(c)]==1?"(":")";
  return result;
}