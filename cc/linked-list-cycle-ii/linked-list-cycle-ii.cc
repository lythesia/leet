#include <cstdio>
#include <iostream>
using namespace std;

struct ListNode {
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(NULL) {}
};
typedef ListNode *link;
class Solution {
public:
  link detectCycle(link head) {
    if(!head || !head->next) return NULL;
    link step1 = head, step2 = head;
    while(step2 && step2->next) {
      step1 = step1->next, step2 = step2->next->next;
      if(step1 == step2) { // has circle
        step1 = head;
        while(step1 != step2) step1 = step1->next, step2 = step2->next;
        return step1;
      } 
    }
    return NULL;
  }
};

int main(int argc, const char *argv[])
{
  
  return 0;
}
