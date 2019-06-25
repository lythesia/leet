#include <cstdio>
#include <iostream>
#include <unordered_map>
using namespace std;

struct RandomListNode {
    int label;
    RandomListNode *next, *random;
    RandomListNode(int x) : label(x), next(NULL), random(NULL) {}
};
typedef RandomListNode *node;
class Solution {
public:
  node copyRandomList(node head) {
    node res = new RandomListNode(0), prev = res;
    unordered_map<node, node> tab;
    while(head) {
      if(tab.count(head) > 0) {       // old -----> head
        prev->next = tab[head];
      }
      else {
        tab[head] = new RandomListNode(head->label);
        prev->next = tab[head];
      }
      if(tab.count(head->random) > 0) { // head->next <---- head
        tab[head]->random = tab[head->random];
      }
      else if(head->random){ // not null in ahead
        tab[head->random] = new RandomListNode(head->random->label);
        prev->next->random = tab[head->random];
      }
      head = head->next, prev = prev->next;
    }
    node tmp = res;
    res = res->next;
    delete tmp;
    return res;
  }
};

int main(int argc, const char *argv[])
{
  
  return 0;
}
