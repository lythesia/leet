#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;


class MedianFinder {
public:

  // Adds a number into the data structure.
  void addNum(int num) {
    // ensure lq >= sq
    if(lq.empty()) lq.push(num);
    else {
      if(num <= lq.top()) {
        if(lq.size() > sq.size()) {
          sq.push(lq.top()); lq.pop();
          lq.push(num);
        }
        else lq.push(num);
      }
      else {
        if(lq.size() > sq.size()) sq.push(num);
        else {
          if(num < sq.top()) lq.push(num);
          else {
            lq.push(sq.top()); sq.pop();
            sq.push(num);
          }
        }
      }
    }
  }

  // Returns the median of current data stream
  double findMedian() {
    if(sq.size() == lq.size()) return (sq.top() + lq.top()) / 2.0;
    else return lq.top();
  }
private:
  priority_queue<int, vector<int>, greater<int>> sq; // for large
  priority_queue<int, vector<int>> lq; // for small
};
