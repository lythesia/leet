#include <cstdio>
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
using namespace std;

typedef vector<int> vi;
class Solution {
public:
  /* dp:
   *   - len[i]: longest ends at `i`
   *   if s[i] == ')': (since if s[i] == '(', len[i] === 0)
   *   - len[i] = len[i-2] + 2: if s[i-1] == '('
   *   -     or = len[i-1] + 2 + len[i - len[i-1] - 2]: if s[i-1] == ')'
   *                        i-x   i-1 i
   *              e.g: ...(  (......) )
   *                                  +2
   *                         +len[i-1]
   *                  i-len[i-1]-2: despite last ')'s pair and len[i-1](since any len[i] is well-matched)
   */
  int longestValidParentheses(string s) {
    if(s.size() < 2) return 0;
    vector<int> len(s.size(), 0);
    int res = 0;
    for(int i=1; i<s.size(); i++) {
      if(s[i] == ')') {
        if(s[i-1] == '(') len[i] = i-2>=0 ? len[i-2]+2 : 2, res = max(res, len[i]);
        else {
          if(i-len[i-1]-1 >= 0 && s[i-len[i-1]-1] == '(') 
            len[i] = len[i-1] + 2 + (i-len[i-1]-2>=0 ? len[i-len[i-1]-2] : 0), res = max(res, len[i]);
        }
      }
    }
    return res;
  }
};

int main(int argc, const char *argv[])
{
  //string s = ")()(((())))(";
  //string s = "()(()";
  //string s = "()";
  //string s = "(()(((()";
  string s = "(()())";
  Solution so;
  cout << so.longestValidParentheses(s) << endl;
  return 0;
}
