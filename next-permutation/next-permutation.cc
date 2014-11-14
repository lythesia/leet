#include <cstdio>
#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

typedef vector<int> vi;

class Solution {
public:
  void nextPermutation(vi &num) {
    int i = num.size()-1;
    for(; i>0; i--) {
      if(num[i-1] < num[i]) break;
    }
    if(i) {
      int j = num.size() - 1;
      for(; j>i-1; j--) if(num[j] > num[i-1]) break;
      swap(num[j], num[i-1]);
      sort(num.begin()+i, num.end());
    }
    else {
      reverse(num.begin(), num.end());
    }
  }
};

int main(int argc, const char *argv[])
{
  int x;
  vi num;
  while(scanf("%d", &x) != EOF) num.push_back(x);
  Solution so;
  so.nextPermutation(num);
  for(int i : num) cout << i << " "; cout << endl;
  return 0;
}
