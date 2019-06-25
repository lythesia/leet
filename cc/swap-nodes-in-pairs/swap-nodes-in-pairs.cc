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
  link swapPairs(link head) {
    if(!head || !head->next) return head;
    link dummy = new ListNode(0), prev = dummy;
    dummy->next = head;
    while(head && head->next) {
      prev->next = head->next;
      link tmp = head->next->next;
      head->next->next = head;
      head->next = tmp;
      // advance
      prev = head, head = head->next;
    }
    if(head) prev->next = head;
    return dummy->next;
  }
};

void see_list(link head) {
  while(head) printf("%d -> ", head->val), head = head->next; puts("NULL");
}
#define N 7
int main(int argc, const char *argv[])
{
  int A[] = {1,4,3,2,5,2,5}, n = N;
  link head = new ListNode(A[0]), p = head;
  while(--n) p->next = new ListNode(A[N-n]), p = p->next;
  see_list(head);
  puts("--------");

  Solution so;
  link res = so.swapPairs(head);
  see_list(res);
  return 0;
}
