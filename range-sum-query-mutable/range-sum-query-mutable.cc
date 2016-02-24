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
      arr = n;
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

  void update(int p, int v) {
    assert(p >=0 && p < len);
    updatest(0, len-1, p, v - arr[p], 0), arr[p] = v;
  }

  void updatest(int s, int e, int p, int d, int i) {
    if(p < s || p > e) return;

    st[i] += d;
    if(s != e) {
      int m = s + (e - s) / 2;
      updatest(s, m, p, d, 2*i+1);
      updatest(m+1, e, p, d, 2*i+2);
    }
  }

private:
  vi arr;
  vi st;
  int len;
};

int main(int argc, const char *argv[])
{
  vi n = {1,3,5};
  NumArray na(n);
  cout << na.sumRange(0, 2) << endl;
  na.update(1, 2);
  cout << na.sumRange(0, 2) << endl;
  return 0;
}
