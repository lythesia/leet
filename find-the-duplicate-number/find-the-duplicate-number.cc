#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

/* 
 * imagine array(n + 1) in range [1, n] a linked list, we access next like a[next], where next = a[curr],
 * if there's duplicate, must exist at least two a[i], a[j] are same, both point to a[k], then we iterate it like
 * linked list:
 * 1. we prove it's cycle: if no cycle, assume we first met a[i], then a[i] = k -> a[k], at some time we'll meet a
 * [j] = k -> a[k](since if no cycle, we can surely visit all), that form cycle
 * 2. now has cycle: so we'll evetually go into it, so the start of the cycle is just the a[k], which previous is
 * the iterator directly prev a[i], and iterate cycle prev a[j]
 */
class Solution {
public:
  int findDuplicate(vi &n) {
    int p1 = 0, p2 = 0;
    while(1) {
      p1 = n[p1], p2 = n[n[p2]];
      if(p1 == p2) {
        p1 = 0;
        while(p1 != p2) p1 = n[p1], p2 = n[p2];
        break;
      }
    }
    return p1;
  }
};

int main(int argc, const char *argv[])
{
  vi n(10);
  iota(n.begin(), n.end(), 1);
  n.push_back(4);
  random_shuffle(n.begin(), n.end());
  Solution so;
  cout << so.findDuplicate(n) << endl;
  return 0;
}
