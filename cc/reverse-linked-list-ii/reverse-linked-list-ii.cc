#include <cstdio>
#include <iostream>
#include <algorithm>
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

  link reverseBetween(link head, int m, int n) {
    if(!head || !head->next) return head;
    link dummy = new ListNode(0);
    dummy->next = head;
    int d = n - m + 1;
    link p = dummy;
    while(--m) p = p->next;
    link pn = p;
    while(d--) pn = pn->next;
    link tmp = p->next;
    link tmp2 = pn->next;
    pn->next = NULL;
    p->next = rev(p->next);   // is this one-pass?
    tmp->next = tmp2;
    return dummy->next;
  }
};

#define N 5
void see_list(link head) {
  while(head) printf("%d -> ", head->val), head = head->next; puts("NULL");
}
int main(int argc, const char *argv[])
{
  int A[] = {1,2,3,4,5}, n = N;
  link head = new ListNode(A[0]), p = head;
  while(--n) p->next = new ListNode(A[N-n]), p = p->next;
  see_list(head);
  puts("--------");

  Solution so;
  link res = so.reverseBetween(head, 2, 4);
  see_list(res);
  return 0;
}
