#include <cstdio>
#include <iostream>
#include <unordered_map>
#include <algorithm>
#include <cassert>
using namespace std;

//#define debug
struct entry {
  int val;
  int prev;
  int next;
  entry(): val(0), prev(0), next(0) {}
  entry(int value, int p, int n): val(value), prev(p), next(n) {}
};
typedef unordered_map<int, entry> mie;

class LRUCache {
public:
  LRUCache(int capacity) {
    cap = capacity;
    newest = 0, oldest = 0;
  }

  int get(int key) {
    auto target = cache.find(key);
    if(target == cache.end()) return -1;

    if(key == oldest) {
      oldest = (target->second).next;
      cache[oldest].prev = oldest;  // or cache[oldest].prev is wrong!
    }
    else {
      cache[(target->second).prev].next = (target->second).next;
      cache[(target->second).next].prev = (target->second).prev;
    }
    if(key != newest) { // or cache[newest].prev is wrong!
      (target->second).next = key;
      (target->second).prev = newest;
      cache[newest].next = key;
      newest = key;
    }
    return (target->second).val;
  }

  void set(int key, int value) {
    if(!cap) return;
    if(cache.empty()) {
      oldest = key, newest = key;
      cache[key] = entry(value, key, key);
    }
    auto target = cache.find(key);
    if(target != cache.end()) {
      get(key);
      (target->second).val = value;
    }
    else { // not in cache
      cache[key] = entry(value, newest, key);
      cache[newest].next = key;
      newest = key;
      assert(key != oldest);
      if(cache.size() > cap) {
        int old_oldest = oldest;
        oldest = cache[oldest].next;
        cache[oldest].prev = oldest;
        cache.erase(old_oldest);
      }
    }
  }

  void echo() {
#ifdef debug
    for(auto i=cache.begin(); i!=cache.end(); i++)
      printf("[%d]=%d(->%d) ", (i->first), (i->second).val, (i->second).next);
    printf("oldest: %d, newest: %d\n\n", oldest, newest);
#endif
  }

  void order() {
#ifdef debug
    assert(cache[oldest].prev == oldest);
    assert(cache[newest].next == newest);
    int cur = oldest;
    printf("oldest: %d, newest: %d\n", oldest, newest);
    while(cur != newest) {
      printf("%d(%d,%d) -> ", cur, cache[cur].prev, cache[cur].next);
      assert(cache[cache[cur].next].prev == cur);
      cur = cache[cur].next;
    }
    printf("%d(%d,%d)\n", cur, cache[cur].prev, cache[cur].next);
#endif
  }
  
  mie cache;
  int cap;
  int newest, oldest;
};

int main(int argc, const char *argv[])
{
  LRUCache c(10);
#if 0
  c.set(1,1);
  c.echo();
  c.set(2,2);
  c.echo();
  c.set(3,3);
  c.echo();
  c.set(4,4);
  c.echo();
  cout << c.get(4) << endl;;
  c.echo();
  cout << c.get(3) << endl;;
  c.echo();
  cout << c.get(2) << endl;;
  c.echo();
  cout << c.get(1) << endl;;
  c.echo();
  c.set(5,5);
  c.echo();
  cout << c.get(1) << endl;;
  c.echo();
  cout << c.get(2) << endl;;
  c.echo();
  cout << c.get(3) << endl;;
  c.echo();
  cout << c.get(4) << endl;;
  c.echo();
  cout << c.get(5) << endl;;
  c.echo();
#endif
  return 0;
}
