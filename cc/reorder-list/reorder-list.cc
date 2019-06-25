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

  /* O(n^2) TLE */
  /*void reorderList(link head) {
    if(!head || !head->next || !head->next->next) return;
    link curr = head->next;
    while(curr) {
      link r = rev(curr);
      head->next = r;
      head = r;
      curr = r->next;
    }
  }*/
  /* O(n): cut list from middle, reverse 2nd part, and insert to 1st every another */
  void reorderList(link head) {
    if(!head || !head->next || !head->next->next) return;
    link step1 = head, step2 = head;
    while(step2 && step2->next) {
      step1 = step1->next;
      step2 = step2->next->next;
    }
    link rev_head = step1->next;
    step1->next = NULL;
    rev_head = rev(rev_head);
    link cur = head;
    while(rev_head) {
      link tmp = rev_head->next;
      rev_head->next = cur->next;
      cur->next = rev_head;
      cur = cur->next->next;
      rev_head = tmp;
    }
  }
};

void see_list(link head) {
  link curr = head;
  cout << "( ";
  while(curr) {
    cout << curr->val << ' ';
    curr = curr->next;
  }
  cout << ")" << endl;
}

#define see_arr(x) for(auto i : (x)) cout << i << ' '; cout << endl
#define N 10
int main(int argc, const char *argv[])
{
  Solution so;
  int n[N];
  for(int i=0; i<sizeof(n)/sizeof(int); i++) n[i] = i;
  random_shuffle(n, n+N);

  link head = new ListNode(n[0]), curr = head;
  for(int i=1; i<N; i++) {
    link tmp = new ListNode(n[i]);
    curr->next = tmp;
    curr = tmp;
  }
  cout << "init:" << endl;
  see_arr(n);
  cout << "------------------" << endl;

  so.reorderList(head);
  see_list(head);
  return 0;
}
