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
  link rotateRight(link head, int k) {
    if(!head || !head->next || !k) return head;
    link p = head, pk = head;
    while(k--) pk = pk->next ? pk->next : head; // allow circle to head again
    while(pk->next) p = p->next, pk = pk->next;
    // [p->next, pk] to begin
    pk->next = head;
    link res = p->next;
    p->next = NULL;
    return res;
  }
};

void see_list(link head) {
  while(head) printf("%d -> ", head->val), head = head->next; puts("NULL");
}
#define N 2
int main(int argc, const char *argv[])
{
  int n = N;
  link head = new ListNode(1), p = head;
  while(--n) p->next = new ListNode(N-n+1), p = p->next;
  see_list(head);
  puts("--------");
  Solution so;
  link res = so.rotateRight(head, 3);
  see_list(res);
  return 0;
}
