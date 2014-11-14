#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <vector>
#include <map>
#include <unordered_map>
#include <utility>
#include <algorithm>
using namespace std;

struct Point {
  int x, y;
  Point(): x(0), y(0) {}
  Point(int a, int b): x(a), y(b) {}
};
typedef vector<Point> vp;
class Solution {
public:
  int gcd(int a, int b) {
    return !b ? a : gcd(b,a%b);
  }

  int maxPoints(vp &points) {
    int res = 0;
    for(int i=0; i<points.size(); i++) {
      map<pair<int,int>, int> k;
      int mmax = 0, same = 0;
      for(int j=i+1; j<points.size(); j++) {
        int x = points[i].x - points[j].x, y = points[i].y - points[j].y;
        if(!x && !y) ++same;
        else if(!x) {
          k[make_pair(0, 1)]++;
          mmax = max(k[make_pair(0, 1)], mmax);
        }
        else {
          int g = gcd((x), (y));
          x/=g, y/=g;
          k[make_pair(x, y)]++;
          mmax = max(k[make_pair(x, y)], mmax);
        }
      }
      res = max(res, mmax+same+1);
    }
    return res;
  }
};

int main(int argc, const char *argv[])
{
  vp ps;
  ps.push_back(Point(0,0));
  ps.push_back(Point(-1,-1));
  ps.push_back(Point(2,2));
  Solution so;
  cout << so.maxPoints(ps) << endl;
  cout << so.gcd(1,1) << endl;
  cout << so.gcd(-1,-1) << endl;
  return 0;
}
