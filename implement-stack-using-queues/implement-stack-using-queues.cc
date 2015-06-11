#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Stack {
public:
  void push(int x) {
    q[cur].push(x);
  }

  void pop() {
    while(q[cur].size() > 1) {
      int t = q[cur].front(); q[cur].pop();
      q[1-cur].push(t);
    }
    q[cur].pop();
    cur = 1 - cur;
  }

  int top() {
    int t = 0;
    while(q[cur].size()) {
      t = q[cur].front(); q[cur].pop();
      q[1-cur].push(t);
    }
    cur = 1 - cur;
    return t;
  }

  bool empty() {
    return q[cur].empty();
  }
private:
  queue<int> q[2];
  int cur = 0;
};

int main(int argc, const char *argv[])
{
  Stack s;
  s.push(2);
  s.push(3);
  s.push(4);
  cout << s.top() << endl;
  s.pop();
  cout << s.top() << endl;
  s.push(5);
  cout << s.top() << endl;
  return 0;
}
