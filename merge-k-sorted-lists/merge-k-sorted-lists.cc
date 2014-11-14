#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

struct ListNode {
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(NULL) {}
};
typedef ListNode *link;
typedef vector<link> vl;
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

  link mergeK(vl &lists, int s, int e) {
    int n = e - s;
    if(n == 0) return NULL;
    if(n == 1) return lists[s];
    return mergeTwoLists(mergeK(lists, s, s+n/2), mergeK(lists, s+n/2, s+n));
  }

  link mergeKLists(vl &lists) {
    return mergeK(lists, 0, lists.size());
  }

  /* 
  // also works
  link mergeKLists2(vl &l) {
    if(l.size() == 0) return NULL;
    if(l.size() == 1) return l[0];
    int n = l.size() / 2;
    vl left(n), right(l.size()-n); // must reserve space exactly same!
    copy(l.begin(), l.begin()+n, left.begin());
    copy(l.begin()+n, l.end(), right.begin());
    return mergeTwoLists(mergeKLists2(left), mergeKLists2(right));
  }
  */
};

void see_list(link head) {
  while(head) printf("%d -> ", head->val), head = head->next; puts("NULL");
}
#define N 32
#define NA 1
#define NB 0
#define NC 1
int main(int argc, const char *argv[])
{
  int A[3][N] = {
    {2},
    {},
    {-1}
  };
  /*
  for(int i=0; i<NA; i++) A[i] = rand() % N;
  sort(A, A+NA);
  for(int i=0; i<NB; i++) B[i] = rand() % N;
  sort(B, B+NB);
  */

  int na = NA;
  link p;
  link l1 = new ListNode(A[0][0]);
  p = l1;
  while(--na) p->next = new ListNode(A[0][NA-na]), p = p->next;
  printf("A: "); see_list(l1);
  //int nb = NB;
  link l2 = NULL;
  printf("B: "); see_list(l2);
  int nc = NC;
  link l3 = new ListNode(A[2][0]);
  p = l3;
  while(--nc) p->next = new ListNode(A[2][NC-nc]), p = p->next;
  printf("C: "); see_list(l3);
  puts("--------");

  vl lists = {l1, l2, l3};
  Solution so;
  link res = so.mergeKLists2(lists);
  see_list(res);
  return 0;
}
