#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  bool addeq(string &a, string &b, string &s) {
    int c = 0, sum = 0;
    int lens = s.length(), lena = a.length(), lenb = b.length(), len = min(lena, lenb);
    --lena, --lenb, --lens;
    for(int i=0; i<len && ~lens; i++, --lena, --lenb, --lens) {
      sum = c + (a[lena] - '0') + (b[lenb] - '0');
      if(sum % 10 != s[lens] - '0') return false;
      c = sum / 10;
    }
    if(lena >=0 && lenb >= 0) return false;

    string *r = lena >=0 ? &a : &b;
    int lenr = lena >= 0 ? lena : lenb;
    for(; ~lenr && ~lens; --lenr, --lens) {
      sum = c + ((*r)[lenr] - '0');
      if(sum % 10 != s[lens] - '0') return false;
      c = sum / 10;
    }
    if(lenr >= 0) return false;
    else if(lens >= 0) {
      // last
      if(!lens) return c == s[lens] - '0';
      else return false;
    }
    else return true;
  }

  bool dfs(string &n, int s1, int l1, int l2) {
    int r3 = n.length() - s1 - l1 - l2, l = max(l1, l2);
    // over
    if(!r3) return s1;
    // left not enough
    if(r3 < l) return false;
    // assert str1 and str2 and str3 valid
    int s3 = s1 + l1 + l2, s2 = s1 + l1;
    if(n[s3] == '0') return false;
    string a = n.substr(s1, l1), b = n.substr(s2, l2), c = n.substr(s3, l);
    if(addeq(a, b, c)) return dfs(n, s2, l2, l);
    else {
      if(l < r3) {
        c = n.substr(s3, l + 1);
        return addeq(a, b, c) && dfs(n, s2, l2, l + 1);
      }
      else return false;
    }
  }

  bool isAdditiveNumber(string n) {
    if(n.length() < 3) return false;
    int len = n.length() / 2;
    for(int l1=1; l1<=len; l1++) {
      for(int l2=1; l2<=len; l2++) {
        if(n[l1] == '0') {
          if(dfs(n, 0, l1, 1)) return true;
          else break; // change l1
        }
        else if(dfs(n, 0, l1, l2)) return true;
      }
    }
    return false;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  // string a = "99",
  //        b = "100",
  //        s = "199";
  // cout << boolalpha << so.addeq(a, b, s) << endl;
  cout << boolalpha << so.isAdditiveNumber("199100199") << endl;
  cout << boolalpha << so.isAdditiveNumber("1234") << endl;
  cout << boolalpha << so.isAdditiveNumber("101") << endl;
  return 0;
}
