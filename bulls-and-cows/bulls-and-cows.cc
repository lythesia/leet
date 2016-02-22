#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  string getHint(string secret, string guess) {
    int h[58] = {0};
    int b = 0, c = 0;
    for(size_t i=0; i<secret.length(); i++) {
      int cs = secret[i], cg = guess[i];
      if(cs == cg) b++;
      else {
        if(h[cs] < 0) c++;
        if(h[cg] > 0) c++;
        h[cs]++, h[cg]--;
      }
    }
    return to_string(b) + "A" + to_string(c) + "B";
  }
};

int main(int argc, const char *argv[])
{
  // string s = "1122",
  //        g = "2211";
  // string s = "1123",
  //        g = "0111";
  string s = "1807",
         g = "7810";
  Solution so;
  cout << so.getHint(s, g) << endl;
  return 0;
}
