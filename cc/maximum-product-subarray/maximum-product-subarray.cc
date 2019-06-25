#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
class Solution {
public:
  // two accum: positive and negative
  // when both exists:
  // if n[i] < 0: swap(pos*=i, neg*=i)
  // else if > 0: pos*=i, neg*=i
  // else: pos = neg = 0;
  // take care when pos/neg not ready
  int maxProduct(vi &ns) {
    if(ns.size() == 1) return ns[0];
    int pos = 0, neg = 0, mmax = 0;
    for(int i : ns) {
      if(i > 0) pos = (pos ? pos*i:i), neg *= i;
      else if(i < 0) {
        if(neg) {
          int t = pos;
          pos = neg*i, neg = t ? t*i:i;
        }
        else neg = (pos ? pos*i:i), pos = 0;
      }
      else pos = neg = 0;
      mmax = max(pos, mmax);
    }
    return mmax;
  }
};

int main(int argc, const char *argv[])
{
  vi v = {2,-5,-2,-4,3};
  // vi v = {2,3,-2,4};
  // vi v = {4,2,0,2,-1,2,-2,9};
  Solution so;
  cout << so.maxProduct(v) << endl;
  return 0;
}
