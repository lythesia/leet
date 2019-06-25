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
  link rm(link &p) {
    if(!p->next || p->val != p->next->val) return p->next;
    while(p->next && p->next->val == p->val) {
      link t = p->next;
      p->next = p->next->next;
      delete t;
    }
    link res = p->next;
    delete p;
    p = NULL;
    return res;
  }

  link deleteDuplicates(link head) {
    if(!head || !head->next) return head;
    link res = new ListNode(0), prev = res;
    bool keep = false;
    while(head) {
      keep = (head->next && head->val == head->next->val);
      link p = rm(head);
      prev->next = head ? head : p;
      prev = keep ? prev : prev->next;
      head = p;
    }
    return res->next;
  }
};

#define N 5
void see_list(link head) {
  while(head) printf("%d -> ", head->val), head = head->next; puts("NULL");
}
int main(int argc, const char *argv[])
{
  int A[] = {1,1,1,2,3,4,5}, n = N;
  link head = new ListNode(A[0]), p = head;
  while(--n) p->next = new ListNode(A[N-n]), p = p->next;
  see_list(head);
  puts("--------");

  Solution so;
  link res = so.deleteDuplicates(head);
  see_list(res);
  return 0;
}
