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
  link removeNthFromEnd(link head, int n) {
    if(!head || n==0) return head;
    link dummy = new ListNode(0);
    dummy->next = head;
    link p = dummy, pn = dummy;
    while(n--) {
      pn = pn->next;
      if(!pn) return head;
    }
    while(pn->next) p = p->next, pn = pn->next;
    p->next = p->next->next;
    return dummy->next;
  }
};

int main(int argc, const char *argv[])
{
  
  return 0;
}
