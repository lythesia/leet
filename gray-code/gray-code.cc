#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
class Solution {
public:
  vi grayCode(int n) {
    if(!n) return { 0 };
    vi ans;
    ans.push_back(0); ans.push_back(1);
    for(int k=1; k<n; k++) for(int i=(1<<k)-1; i>=0; i--) ans.push_back((1<<k) | ans[i]); // do it reversely to make it graycode with first number!
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  int n = 4;
  for(int i : so.grayCode(n)) cout << i << " "; cout << endl;
  return 0;
}
