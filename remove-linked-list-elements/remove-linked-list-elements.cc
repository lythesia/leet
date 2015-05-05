#include <bits/stdc++.h>
using namespace std;

struct ListNode {
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(NULL) {}
};
typedef ListNode *node;
class Solution {
public:
  node removeElements(node head, int val) {
    ListNode dummy(0);
    node prev = &dummy;
    dummy.next = head;
    while(head) {
      node next = head->next;
      if(head->val == val) {
        prev->next = head->next;
      } 
      else prev = head;
      head = next;
    }
    return dummy.next;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  ListNode n0(0), n1(1);
  node h = &n0;
  n0.next = &n1;
  assert(so.removeElements(h, 1) == &n0 && n0.next == NULL);
  return 0;
}
