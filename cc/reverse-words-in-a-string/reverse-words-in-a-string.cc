#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  // try ruby!
  void reverseWords(string &s) {
    size_t p = 0, e = 0;
    string t;
    while((p = s.find_first_not_of(' ', e)) != string::npos) {
      e = s.find_first_of(' ', p);
      t = s.substr(p, e-p) + " " + t;
    }
    if(!t.empty()) t.pop_back();
    s.swap(t);
  }
};

int main(int argc, const char *argv[])
{
  string s = "  the  sky is   blue ";
  // string s = "";
  Solution so;
  so.reverseWords(s);
  cout << '"' << s << '"' << endl;
  return 0;
}
