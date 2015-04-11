#include <bits/stdc++.h>
using namespace std;

class MinStack {
public:
  void push(int x) {
    st.push_back(x);
    if(minst.empty() || x <= minst.back()) minst.push_back(x);
  }

  void pop() {
    if(!minst.empty() && st.back() == minst.back()) minst.pop_back();
    st.pop_back();
  }

  int top() {
    return st.back();
  }

  int getMin() {
    return minst.back();
  }
private:
  vector<int> st;
  vector<int> minst;
};

int main(int argc, const char *argv[])
{
  MinStack st;
  st.push(2);
  cout << st.top() << endl;
  cout << st.getMin() << endl;
  cout << "----" << endl;

  st.push(3);
  cout << st.top() << endl;
  cout << st.getMin() << endl;
  cout << "----" << endl;

  st.push(1);
  cout << st.top() << endl;
  cout << st.getMin() << endl;
  cout << "----" << endl;
  return 0;
}
