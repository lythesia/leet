#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  int findKthLargest(vi n, int k) {
    priority_queue<int, vi, greater<int>> heap;
    for(int i : n) {
      if((int)heap.size() >= k) {
        if(i > heap.top()) heap.pop();
        else continue;
      }
      heap.push(i);
    }
    return heap.top();
  }
};

int main(int argc, const char *argv[])
{
  int k = 2;
  vi v = {3,2,1,5,6,4};
  Solution so;
  cout << so.findKthLargest(v, k) << endl;
  return 0;
}
