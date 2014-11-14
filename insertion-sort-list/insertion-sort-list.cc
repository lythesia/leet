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
  link insertionSortList(link head) {
    if(!head || !head->next) return head;
    link dummy = new ListNode(0);
    dummy->next = head;

    for(link i=head; i->next; i=i->next) {
      link mmin = i;
      for(link j=i->next; j; j=j->next) {
        if(j->val < mmin->val) mmin = j;
      }
      swap(i->val, mmin->val);
    }
    // free
    link res = dummy->next;
    delete dummy;
    return res;
  }
};

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

  link res = so.insertionSortList(head);
  see_list(res);
  return 0;
}
