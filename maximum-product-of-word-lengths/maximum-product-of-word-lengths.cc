#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  int maxProduct(vs words) {
    int len = words.size();
    vi bit(len);
    for(int i=0; i<len; i++) for(char c : words[i]) bit[i] |= (1 << (c - 'a'));
    int ans = 0;
    for(int i=0; i<len-1; i++) for(int j=1; j<len; j++)
        if(!(bit[i] & bit[j])) ans = max(ans, int(words[i].length()*words[j].length()));
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  vs w = {"abcw", "baz", "foo", "bar", "xtfn", "abcdef"};
  cout << so.maxProduct(w) << endl;
  return 0;
}
