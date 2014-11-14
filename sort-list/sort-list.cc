#include <cstdio>
#include <iostream>
#include <algorithm>
#include <random>
using namespace std;

struct ListNode {
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(NULL) {}
};
typedef ListNode *link;
void see_list(link head) {
  link curr = head;
  cout << "( ";
  while(curr) {
    cout << curr->val << ' ';
    curr = curr->next;
  }
  cout << ")" << endl;
}
class Solution {
public:
  inline bool empty(link head) { return head->next == NULL; } 

  void splice_after(link pos, link before_first, link before_last) {
    link first = before_first->next,
         after = pos->next;
    before_first->next = before_last->next;
    pos->next = first;
    before_last->next = after;
  }

  link merge_list(link l1, link l2) {
    link first1 = l1,
         first2 = l2;
    while(first1->next && first2->next) {
      if(first2->next->val < first1->next->val) {
        splice_after(first1, first2, first2->next);
      }
      first1 = first1->next;;
    }
    if(first2->next) {
      first1->next = first2->next;
      first2->next = NULL;
    }
    return l1;
  }

  link sortList(link head) {
    if(!head || !head->next) return head;
    link dummy = new ListNode(0);
    dummy->next = head;
    link carry = new ListNode(0);
    link counter[64];
    for(link &l : counter) l = new ListNode(0);

    int fill = 0;
    while(!empty(dummy)) {
      splice_after(carry, dummy, dummy->next);
      int i = 0;
      while(i < fill && !empty(counter[i])) {
        merge_list(counter[i], carry);
        swap(carry, counter[i]);
        ++i;
      }
      swap(carry, counter[i]);
      if(i == fill) ++fill;
    }
    for(int i=1; i<fill; i++) merge_list(counter[i], counter[i-1]);
    swap(dummy, counter[fill-1]);

    link res = dummy->next;
    // free
    for(link l : counter) delete l;
    delete carry;
    delete dummy;

    return res;
  }
};

#define see_arr(x) for(auto i : (x)) cout << i << ' '; cout << endl
#define N 64

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

  link res = so.sortList(head);
  see_list(res);
  return 0;
}
