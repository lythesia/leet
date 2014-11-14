#include <cstdio>
#include <cmath>
#include <climits>
#include <iostream>

using namespace std;

class Solution {
public:
  bool isPalindrome(int x) {
    if(x < 0) return false;

    int n = digits(x);
    for(int i=1; i<=n/2; i++)
      if(dig(x, i) != dig(x, n+1-i)) return false;
    return true;
  }

  int dig(int x, int n) {
    return (x / static_cast<int>(pow(10, n-1))) % 10;
  }

  int digits(int x) {
    int res = 0;
    while(x) {
      ++res;
      x/=10;
    }
    return res;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  cout << INT_MAX << endl;
  cout << INT_MIN << endl;
  cout << (-2147447412 >= INT_MIN && -2147447412 <= INT_MAX) << endl;
  cout << so.isPalindrome(-2147447412) << endl;

  return 0;
}
