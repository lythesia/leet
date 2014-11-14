#include <cstdio>
#include <iostream>
#include <string>
#include <stack>
using namespace std;

class Solution {
public:
  bool isValid(string s) {
    stack<char> stak;
    for(char c : s) {
      switch(c) {
        case '(': case '[': case '{':
          stak.push(c);
          break;
        case ')': case ']': case '}':
          if(!stak.empty()) {
            char t = stak.top();
            if((c == ')' && t == '(') ||
               (c == ']' && t == '[') ||
               (c == '}' && t == '{')) stak.pop();
            else return false;
          }
          else return false;
          break;
      }
    }
    return stak.empty();
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  cout << so.isValid("()]")<< endl;
  return 0;
}
