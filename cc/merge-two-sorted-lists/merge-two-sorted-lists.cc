#include <cstdio>
#include <cstdlib>
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
  link mergeTwoLists(link l1, link l2) {
    if(!l1) return l2;
    if(!l2) return l1;
    link d1 = new ListNode(0), d2 = new ListNode(0);
    d1->next = l1, d2->next = l2;
    link p1 = d1, p2 = d2;
    while(p1->next && p2->next) {
      if(p2->next->val < p1->next->val) {
        // slice p2->next to p1 => () => p1->next
        link tmp = p2->next;
        p2->next = p2->next->next;
        tmp->next = p1->next;
        p1->next = tmp;
      }
      p1 = p1->next;
    }
    if(p2->next) p1->next = p2->next;
    return d1->next;
  }
};

void see_list(link head) {
  while(head) printf("%d -> ", head->val), head = head->next; puts("NULL");
}
#define N 32
#define NA 4
#define NB 8
int main(int argc, const char *argv[])
{
  int A[N], B[N];
  for(int i=0; i<NA; i++) A[i] = rand() % N;
  sort(A, A+NA);
  for(int i=0; i<NB; i++) B[i] = rand() % N;
  sort(B, B+NB);

  int na = NA;
  link p;
  link l1 = new ListNode(A[0]);
  p = l1;
  while(--na) p->next = new ListNode(A[NA-na]), p = p->next;
  printf("A: "); see_list(l1);
  int nb = NB;
  link l2 = new ListNode(B[0]);
  p = l2;
  while(--nb) p->next = new ListNode(B[NB-nb]), p = p->next;
  printf("B: "); see_list(l2);
  puts("--------");

  Solution so;
  link res = so.mergeTwoLists(l1, l2);
  see_list(res);
  return 0;
}
