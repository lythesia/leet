#include <cstdio>
#include <iostream>

using namespace std;

struct ListNode {
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
public:
  ListNode *addTwoNumbers(ListNode *l1, ListNode *l2) {
    ListNode *res = new ListNode(0), *p = res;
    ListNode *i = l1, *j = l2;
    int c = 0;
    for(; i && j; i=i->next,j=j->next) {
      int s = i->val + j->val + c;
      c = s / 10;
      p->next = new ListNode(s % 10);
      p = p->next;
    }
    while(i) {
      int s = i->val + c;
      c = s / 10;
      p->next = new ListNode(s % 10);
      p = p->next;
      i = i->next;
    }
    while(j) {
      int s = j->val + c;
      c = s / 10;
      p->next = new ListNode(s % 10);
      p = p->next;
      j = j->next;
    }
    if(c) p->next = new ListNode(c);
    return res->next;
  }
};

ListNode *to_list(int n) {
  ListNode *res = new ListNode(0), *p = res;
  while(n) {
    p->next = new ListNode(n%10);
    p = p->next;
    n /= 10;
  }
  return res;
}

void plist_r(ListNode *l) {
  if(!l) return;
  plist_r(l->next);
  printf("%d", l->val);
}

void free_list(ListNode *l) {
  while(l) {
    ListNode *t = l;
    l = l->next;
    delete t;
  }
}

int main(int argc, const char *argv[])
{
  Solution so;
  int a, b;
  while(scanf("%d%d", &a, &b) != EOF) {
    ListNode *la = to_list(a), *lb = to_list(b);
    ListNode *aa = a ? la->next : la, *bb = b ? lb->next : lb;
    //plist_r(aa); puts("");
    //plist_r(bb); puts("");
    ListNode *res = so.addTwoNumbers(aa, bb);
    plist_r(res); printf(" = "); plist_r(aa); printf(" + "); plist_r(bb); puts("");
    free_list(la);
    free_list(lb);
    free_list(res);
  }
  return 0;
}
