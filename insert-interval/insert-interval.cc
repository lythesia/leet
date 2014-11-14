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
void see(vector<Interval> &vp);

typedef vector<Interval>::iterator iter;
class Solution {
public:
  vector<Interval> insert(vector<Interval> &ins, Interval in) {
    vector<Interval> res;
    bool inserted = false;
    iter it = ins.begin();
    for(; it!=ins.end(); it++) {
      if(inserted) {
        res.push_back(*it);
        continue;
      }
      if(in.start <= it->start) {
        if(in.end < it->start) {
          res.push_back(in);
          res.push_back(*it);
          inserted = true;
          continue;
        }
        else if(in.end == it->start) {
          res.push_back(Interval(in.start, it->end));
          inserted = true;
          continue;
        }
        else if(in.end <= it->end) {
          res.push_back(Interval(in.start, it->end));
          inserted = true;
        }
        else continue;
      }
      else { // in.s > it.s
        if(in.start < it->end) {
          if(in.end <= it->end) {
            res.push_back(*it);
            inserted = true;
            continue;
          }
          else in.start = it->start;
        }
        else if(in.start == it->end) in.start = it->start;
        else res.push_back(*it);
      }
    }
    if(!inserted) res.push_back(in);
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
    Interval(1, 2),
    Interval(3, 5),
    Interval(6, 7),
    Interval(8, 10),
    Interval(12, 16)
  };
  Interval in(17, 18);

  Solution so;
  vector<Interval> res = so.insert(vp, in);
  see(res);
  return 0;
}
