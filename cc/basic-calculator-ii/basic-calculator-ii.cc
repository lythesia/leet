#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

#define __tok__(c) tok((c) == '+' ? PLUS : (c) == '-' ? MINUS : (c) == '*' ? MUL : DIV)
class Solution {
public:
  enum {
    PLUS,
    MINUS,
    MUL,
    DIV,
    NUM,
  };
  struct tok {
    int type;
    int val;
    tok(int t, int v = 0) : type(t), val(v) {}
  };

  bool precede(char x, char y) {
    if(y == '+' || y == '-') return x == '*' || x == '/';
    else return false;
  }
  int calculate(string s) {
    // infix to postfix
    vector<tok> post;
    stack<char> op;
    int p = 0, len = s.length();
    while(p < len) {
      if(isblank(s[p])) while(isblank(s[p])) p++;
      else if(isdigit(s[p])) {
        string ns;
        while(isdigit(s[p])) ns.push_back(s[p++]);
        post.emplace_back(tok{NUM, atoi(ns.c_str())});
      }
      else {
        while(!op.empty() && !precede(s[p], op.top())) {
          post.emplace_back(__tok__(op.top()));
          op.pop();
        }
        op.emplace(s[p++]);
      }
    }
    while(!op.empty()) {
      post.emplace_back(__tok__(op.top()));
      op.pop();
    }
    // compute
    stack<int> st;
    for(tok &t : post) {
      if(t.type == NUM) st.push(t.val);
      else {
        int n2 = st.top(); st.pop();
        int n1 = st.top(); st.pop();
        st.push(t.type == PLUS ? n1 + n2 : t.type == MINUS ? n1 - n2 : t.type == MUL ? n1 * n2 : n1 / n2);
      }
    }
    assert(st.size() == 1);
    return st.top();
  }
};

int main(int argc, const char *argv[])
{
  vs v = {
    "3+2*2",
    " 3/2 ",
    " 3+5 / 2   ",
  };
  Solution so;
  for(auto &s : v) cout << s << " = " << so.calculate(s) << endl;
  return 0;
}
