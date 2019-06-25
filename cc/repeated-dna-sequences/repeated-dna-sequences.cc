#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  // encode reversely
  static string todna(int code) {
    static char tab[4] = {'A', 'C', 'G', 'T'};
    static int mask = 3;
    string ans;
    for(int i=0; i<10; i++) {
      ans.push_back(tab[code & mask]);
      code >>= 2;
    }
    return ans;
  }
  static int tocode(char ch) {
    int c = 0;
    switch(ch) {
      case 'A': c = 0; break;
      case 'C': c = 1; break;
      case 'G': c = 2; break;
      case 'T': c = 3; break;
    }
    return c;
  }
  static int tocode(string dna) {
    int ans = 0;
    for(size_t i=0; i<dna.length(); i++) {
      int c = tocode(dna[i]);
      ans |= (c << 2*i);
    }
    return ans;
  }

  vs findRepeatedDnaSequences(string s) {
    vs ans;
    if(s.length() < 10) return ans;
    unordered_map<int, int> h;
    string ss = s.substr(0, 10);
    int cc = tocode(ss);
    h[cc] = 1;
    for(size_t i=10; i<s.length(); i++) {
      int c = tocode(s[i]);
      cc = (cc >> 2) | (c << 18);
      if(h[cc] == 1) ans.push_back(todna(cc));
      h[cc]++;
    }
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  // string t = "AAAAACCCCC"; // 01010101010000000000
  // int c = Solution::tocode(t);
  // printf("code:%x, recover: %s\n", c, Solution::todna(c).c_str());
  string s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT";
  // string s = "AAAAAAAAAAAA";
  Solution so;
  vs ans = so.findRepeatedDnaSequences(s);
  copy(ans.begin(), ans.end(), ostream_iterator<string>(cout, "\n"));
  return 0;
}
