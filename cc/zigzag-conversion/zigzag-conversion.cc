#include <bits/stdc++.h>
using namespace std;

typedef pair<int, char> pic;
typedef priority_queue<pic, vector<pic>, greater<pic>> pq;
class Solution {
public:
  string convert(string s, int n) {
    if(s == "" || n == 1) return s;
    int len = s.length();
    string ans(len, 0);
    pq Q;
    int row = 0, col = 0, dir = 0; // 1 up; 0 down
    int np = len / (n + n -2), nr = len % (n + n - 2), nc = np*(n-1) + nr/n + nr%n;
    for(int i=0; i<len; i++) {
      Q.push(pic(row*nc + col, s[i]));
      if(!dir) {  // down
        if(row == n-1) row--, col++, dir = 1;
        else row++;
      }
      else {      // up
        if(!row) row++, dir = 0;
        else row--, col++;
      }
    }
    int p = 0;
    while(!Q.empty()) {
      ans[p++] = Q.top().second;
      Q.pop();
    }
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  string s("PAYPALISHIRING");
  Solution so;
  cout << so.convert(s, 3) << endl;
  return 0;
}
