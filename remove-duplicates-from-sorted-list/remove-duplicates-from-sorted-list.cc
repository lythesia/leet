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
  link deleteDuplicates(link head) {
    if(!head || !head->next) return head;
    link prev = head, res = head;
    head = head->next;
    while(head) {
      if(prev->val == head->val) {
        // remove head
        prev->next = head->next;
        delete head;
        head = prev;
      }
      prev = head, head = head->next;
    }
    return res;
  }
};

#define N 2
void see_list(link head) {
  while(head) printf("%d -> ", head->val), head = head->next; puts("NULL");
}
int main(int argc, const char *argv[])
{
  int A[] = {1,1,2,3,3}, n = N;
  link head = new ListNode(A[0]), p = head;
  while(--n) p->next = new ListNode(A[N-n]), p = p->next;
  see_list(head);
  puts("--------");

  Solution so;
  link res = so.deleteDuplicates(head);
  see_list(res);
  return 0;
}
