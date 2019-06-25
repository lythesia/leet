#include <cstdio>
#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

typedef long long ll;
class Solution {
public:
  int sqrt(int x) {
    ll l = 1, h = x;
    while(l < h) {
      ll mid = (l+h)/2;
      ll t = mid*mid;
      if(mid == l || mid == h) break;
      if(t < x) {
        l = mid;
      }
      else if(t > x){
        h = mid;
      }
      else return mid;
    }
    return min(l,h);
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  cout << so.sqrt(2147395599) << endl;
  return 0;
}
