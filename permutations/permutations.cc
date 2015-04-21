#include <bits/stdc++.h>
using namespace std;

typedef vector<int> vi;
typedef vector<vi> vvi;
class Solution {
public:
  vvi permute(vi &num) {
    vvi ans;
    sort(num.begin(), num.end());
    do { ans.push_back(num); } while(np(num.begin(), num.end()));
    return ans;
  }

  // implemenet next_permutation
  // only change position of a "digit" when everything to the right is desc, since if not, there's still more
  // permutations "larget" than current
  // so we do this:
  // from end on, we find the desc order, if once on k it breaks desc order, we find the next largest(>a[k])
  // among k+1..n, and swap it with a[k], then reverse k+1..n get the initial asc for new a[k], this is because
  // after swap, k+1..n is still desc
  template <typename I>
    bool np(I begin, I end) {
      // length: 0 or 1
      if(begin == end) return false;
      I i = end;
      if(begin == --i) return false;

      while(true) {
        I j = i;
        if(*--i < *j) { // from end on find "digit" whose right part is desc
          I next = end;
          while(!(*i < *--next));
          iter_swap(i, next);
          reverse(j, end);
          return true;
        }
        if(i == begin) { // not found, all over
          reverse(begin, end); // restore origin
          return false;
        }
      }
    }
};

int main(int argc, const char *argv[])
{
  vi num = { 0,-1,1 };
  Solution so;
  for(vi &v : so.permute(num)) {
    for(int i : v) cout << i << ' '; cout << endl;
  }
  // for(int i : num) cout << i << ' '; cout << endl;
  return 0;
}
