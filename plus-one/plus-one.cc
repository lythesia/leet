#include <cstdio>
#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

#define see(x) cout << #x << ": " << (x) << endl

class Solution {
public:
  vector<int> plusOne(vector<int> &digits) {
    int c = 1;
    vector<int> res;
    int n = find_if(digits.begin(), digits.end(), [](int i){return i!=0;}) - digits.begin();
    for(int i=digits.size()-1; i>=n; i--) {
      int t = digits[i] + c;
      res.insert(res.begin(), t % 10);
      c = t / 10;
    }
    if(c) res.insert(res.begin(), c);
    return res;
  }
};

int main(int argc, const char *argv[])
{
  vector<int> vi = {0, 8};
  Solution so;
  vector<int> res = so.plusOne(vi);
  for(auto i : res) cout << i; cout << endl;
  return 0;
}
