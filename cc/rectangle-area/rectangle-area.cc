#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Solution {
public:
  int computeArea(int A, int B, int C, int D, int E, int F, int G, int H) {
    if(A > C) swap(A, C); if(B > D) swap(B, D);
    if(E > G) swap(E, G); if(F > H) swap(F, H);
    int s1 = (C-A)*(D-B), s2 = (G-E)*(H-F);
    /* no overlap */
    // left    below   right   above
    if(G<=A || H<=B || C<=E || D<=F) return s1 + s2;
    /* overlap */
    else {
      int x1 = max(A, E), y1 = max(B, F),
          x2 = min(C, G), y2 = min(D, H);
      return s1 - (x2-x1)*(y2-y1) + s2;
    }
  }
};

int main(int argc, const char *argv[])
{
  
  return 0;
}
