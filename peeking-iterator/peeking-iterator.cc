#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class Iterator {
  vi::iterator it, last;
public:
  Iterator(const vi &n) it(n.begin()), last(n.end()) {}
  // Iterator(const Iterator &it);
  virtual ~Iterator() {};

  int next() {
    return *it++;
  }
  bool hasNext() const {
    return it != last;
  };
};

class PeekingIterator : public Iterator {
public:
  PeekingIterator(const vi &n) : Iterator(n) {
    peeked = false;
  }

  int peek() {
    if(!peeked) cache = Iterator::next(), peekd = true;
    return cache;
  }
  int next() {
    if(!peeked) cache = Iterator::next();
    else peeked = false;
    return cache;
  }
  int hasNext() {
    return peeked || Iterator::hasNext();
  }
private:
  bool peeked;
  int cache;
};
