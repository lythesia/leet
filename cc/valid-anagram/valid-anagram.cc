#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  bool isAnagram(string s, string t) {
    vi hsh(128, 0);
    for(char c : s) hsh[c]++;
    for(char c : t) hsh[c]--;
    for(char c='a'; c<='z'; c++) if(hsh[c]) return false;
    return true;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  cout << boolalpha << so.isAnagram("rat", "car") << endl;
  cout << boolalpha << so.isAnagram("anagram", "nagaram") << endl;
  return 0;
}
