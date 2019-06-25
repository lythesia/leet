#include <cstdio>
#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

 struct Interval {
     int start;
     int end;
     Interval() : start(0), end(0) {}
     Interval(int s, int e) : start(s), end(e) {}
 };

/*
 * note [1,4] [4,5] => [1,5]
 */
class Solution {
public:
  vector<Interval> merge(vector<Interval> &intervals) {
    vector<Interval> res;
    sort(intervals.begin(), intervals.end(), [](const Interval &p1, const Interval &p2){return p1.start < p2.start;});
    vector<int> sp;
    for(auto i : intervals) {
      if(sp.empty()) {
        sp.push_back(i.start);
        sp.push_back(i.end);
      }
      else if(sp.back() < i.start) {
        int e = sp.back();
        sp.pop_back();
        int s = sp.back();
        sp.pop_back();
        res.push_back(Interval(s, e));
        sp.push_back(i.start);
        sp.push_back(i.end);
      }
      else if(sp.back() == i.start) {
        sp.pop_back();
        sp.push_back(i.end);
      }
      else if(sp.back() > i.start) {
        if(sp.back() >= i.end) continue;
        else {
          sp.pop_back();
          sp.push_back(i.end);
        }
      }
    }
    if(!sp.empty()) {
      int e = sp.back();
      sp.pop_back();
      int s = sp.back();
      sp.pop_back();
      res.push_back(Interval(s, e));
    }
    return res;
  }
};

void see(vector<Interval> &vp) {
  for(auto i : vp) {
    printf("[%d,%d] ", i.start, i.end);
  }
  puts("");
}

int main(int argc, const char *argv[])
{
  vector<Interval> vp = {
    Interval(0, 2),
    Interval(1, 4),
    Interval(3, 5)
  };

  Solution so;
  vector<Interval> res = so.merge(vp);
  see(res);
  return 0;
}
