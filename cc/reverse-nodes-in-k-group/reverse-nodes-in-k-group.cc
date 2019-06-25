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
  link rev(link head) {
    if(!head || !head->next) return head;
    link prev = head, curr = head->next;
    while(curr) {
      link tmp = curr->next;
      curr->next = prev;
      prev = curr, curr = tmp;
    }
    head->next = NULL;
    return prev;
  }

  link reverseKGroup(link head, int k) {
    if(!head || !head->next || k == 0 || k == 1) return head;
    link dummy = new ListNode(0), prev = dummy;
    dummy->next = head;
    link headk = dummy;
    do {
      // advance k
      for(int i=0; i<k; i++) {
        headk = headk->next;
        if(!headk->next) {
          if(i < k-1) {         // if len < k, then do nothing
            prev->next = head;
            return dummy->next;
          }
          else {
            prev->next = rev(head);
            return dummy->next;
          }
        }
      }
      link tmp = headk->next;
      headk->next = NULL;
      prev->next = rev(head);
      head->next = tmp;
      prev = head, head = tmp, headk = prev;
    } while(headk);
    return dummy->next;
  }
};

void see_list(link head) {
  while(head) printf("%d -> ", head->val), head = head->next; puts("NULL");
}
#define N 4
int main(int argc, const char *argv[])
{
  int A[] = {1,2,3,4,5,2,5}, n = N;
  link head = new ListNode(A[0]), p = head;
  while(--n) p->next = new ListNode(A[N-n]), p = p->next;
  see_list(head);
  puts("--------");

  Solution so;
  link res = so.reverseKGroup(head, 2);
  see_list(res);
  return 0;
}
