#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

typedef vector<string> vs;
typedef vector<int> vi;
class Solution {
public:
  int evalRPN(vs &tokens) {
    vi stak;
    for(auto s : tokens) {
      if(s == "-" || s == "+" || s == "*" || s == "/") {
        auto s2 = stak.back(); stak.pop_back();
        auto s1 = stak.back(); stak.pop_back();
        stak.push_back(s == "-" ? s1-s2: 
                       s == "+" ? s1+s2:
                       s == "*" ? s1*s2:s1/s2);
      }
      else stak.push_back(atoi(s.c_str()));
    }
    return stak.back();
  }
};

int main(int argc, const char *argv[])
{
  vs tks = {"4", "13", "5", "/", "+"};
  Solution so;
  cout << so.evalRPN(tks) << endl;
  return 0;
}
