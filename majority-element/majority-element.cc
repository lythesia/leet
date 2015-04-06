#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  /*
   * say k is the # > [ n /2 ] number, so two appearances of k cannot gap more than 2(pigeon's hole), so:
   * 1. if distributed non-even: 
   *    a. if mainly on left, then in worst it ends with cnt == 0, while last is the k(unchanged)
   *    b. if on right, it's trivial
   * 2. if evenly(every two has a k):
   *    then at n-1 th, cnt == 0, last is some other than k, at for n-th becomes cnt == 1, last = k
   */
  int majorityElement(vector<int> &num) {
    int cnt = 0, last = 0;
    for(int i : num) !cnt || i == last ? (cnt++, last = i) : cnt--;
    return last;
  }
};
