#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class NumArray {
public:
  NumArray(vi &n) {
    len = n.size();
    if(len) {
      st = vi(len*4);
      makest(n, 0, len-1, 0);
    }
  }

  int makest(vi &n, int s, int e, int i) {
    if(s == e) return st[i] = n[s];
    else {
      int m = s + (e - s) / 2;
      return st[i] = makest(n, s, m, i*2+1) + makest(n, m+1, e, i*2+2);
    }
  }

  int sumRange(int i, int j) {
    if(i < 0 || j > len-1) return 0;
    if(i > j) swap(i, j);
    return queryst(0, len-1, i, j, 0);
  }

  int queryst(int s, int e, int qs, int qe, int i) {
    if(e < qs || s > qe) return 0;
    else if(qs <= s && e <= qe) return st[i];
    else {
      int m = s + (e - s) / 2;
      return queryst(s, m, qs, qe, 2*i+1) + queryst(m+1, e, qs, qe, 2*i+2);
    }
  }

private:
  vi st;
  int len;
};

int main(int argc, const char *argv[])
{
  // vi n = {-2, 0, 3, -5, 2, -1};
  NumArray na(n);
  // cout << na.sumRange(0, 2) << endl;
  // cout << na.sumRange(2, 5) << endl;
  // cout << na.sumRange(0, 5) << endl;
  return 0;
}
