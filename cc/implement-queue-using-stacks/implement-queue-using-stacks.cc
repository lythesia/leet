#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Queue {
public:
  void push(int x) {
    in.push(x);
  }
  void pop() {
    if(out.empty()) in2out();
    out.pop();
  }
  int peek() {
    if(out.empty()) in2out();
    return out.top();
  }
  bool empty() {
    return in.empty() && out.empty();
  }
protected:
  void in2out() {
    while(!in.empty()) {
      out.push(in.top());
      in.pop();
    }
  }
private:
  stack<int> in, out;
};

int main(int argc, const char *argv[])
{
  Queue q;
  q.push(1);
  q.push(2);
  cout << q.peek() << endl;
  q.pop();
  cout << q.peek() << endl;
  q.pop();
  cout << boolalpha << q.empty() << endl;
  return 0;
}
