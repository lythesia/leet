#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

typedef pair<int,int> pii;
typedef vector<pii> vp;
class Solution {
public:
  /*
   * line segments:
   *      -----
   *  -----
   *    -----
   *        -----
   * sort primary by x-coord, if equal:
   * 1. both left: make smaller height first
   * 2. both right: make larger height first
   * 3. left & right: make left first
   * so if left's height is negative, it meets all above
   */
  vp getSkyline(vvi &bs) {
    vp lines, ans;
    for(auto &v : bs) {
      lines.push_back({v[0], -v[2]});
      lines.push_back({v[1],  v[2]});
    }
    sort(lines.begin(), lines.end());

    multiset<int> heap;
    heap.insert(0);
    int prev = 0, curr = 0;
    for(auto &l : lines) {
      // left
      if(l.second < 0) heap.insert(-l.second);
      else heap.erase(heap.find(l.second)); // only remove one line
      curr = *heap.rbegin(); // largest height
      if(curr != prev) {
        ans.push_back({l.first, curr});
        prev = curr;
      }
    }
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  vvi vv = {
    {2,9,10},
    {3,7,15},
    {5,12,12},
    {15,20,10},
    {19,24,8},
  };
  Solution so;
  vp ans = so.getSkyline(vv);
  for(auto &p : ans) cout << "(" << p.first << "," << p.second << ") "; cout << endl;
  return 0;
}
