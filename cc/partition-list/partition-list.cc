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
  link partition(link head, int x) {
    if(!head || !head->next) return head;
    link left = new ListNode(0), right = new ListNode(0);
    link p = right, prev = left;
    left->next = head;
    while(head) {
      if(head->val >= x) {
        // slice head to right
        prev->next = head->next;
        head->next = NULL;
        p->next = head;
        p = p->next;
        head = prev->next;
      }
      else prev = head, head = head->next;
    }
    prev->next = right->next;
    return left->next;
  }
};

void see_list(link head) {
  while(head) printf("%d -> ", head->val), head = head->next; puts("NULL");
}
#define N 6
int main(int argc, const char *argv[])
{
  int A[] = {1,4,3,2,5,2,5}, n = N;
  link head = new ListNode(A[0]), p = head;
  while(--n) p->next = new ListNode(A[N-n]), p = p->next;
  see_list(head);
  puts("--------");

  Solution so;
  link res = so.partition(head, 3);
  see_list(res);
  return 0;
}
