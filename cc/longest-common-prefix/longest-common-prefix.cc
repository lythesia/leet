#include <cstdio>
#include <iostream>
#include <cstring>
#include <string>
#include <vector>

using namespace std;

#define see(x) cout << #x << ": " << (x) << endl

class Solution {
public:
  string longestCommonPrefix(vector<string> &strs) {
    string res = "";
    char ch;
    int index = 0;
    bool done = false;

    if(strs.empty()) return res;

    while(!done) {
      // 1st
      if(strs[0].length() <= index) break;
      ch = strs[0].at(index);

      for(int i=1; i<strs.size(); i++) {
        string &s = strs[i];
        if(s.length() <= index) {
          done = true;
          break;
        }
        else if(s[index] != ch) {
          done = true;
          break;
        }
      }
      if(!done) {
        res += ch;
        index++;
      }
    }
  return res;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  char buf[128];
  vector<string> vs;

  while(fgets(buf, sizeof(buf), stdin)) {
    string s(buf, strlen(buf) - 1);
    vs.push_back(s);
  }

  //for(auto s : vs) cout << s << endl;

  cout << "lcp:" << endl;
  cout << '"' << so.longestCommonPrefix(vs) << '"' << endl;
  return 0;
}
