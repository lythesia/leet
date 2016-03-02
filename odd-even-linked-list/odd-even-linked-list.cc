#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

struct ListNode {
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(NULL) {}
};

typedef ListNode *link;

class Solution {
public:
  link oddEvenList(link h) {
    if(!h || !h->next) return h;
    link odd = h, even = h->next, h2 = even;
    for(link p=even->next; p; p=p->next->next) {
      odd->next = p, odd = p;
      even->next = p->next, even = p->next;
      if(!p->next) break;
    }
    odd->next = h2;
    return h;
  }
};

void see_list(link head) {
  while(head) printf("%d -> ", head->val), head = head->next; puts("NULL");
}

int main(int argc, const char *argv[])
{
  Solution so;
  ListNode n[5] = {
    ListNode(1),
    ListNode(2),
    ListNode(3),
    ListNode(4),
    ListNode(5),
  };
  for(int i=0; i<4; i++) n[i].next = &n[i+1];
  link head = &n[0];
  see_list(head);
  cout << "----" << endl;

  link ans = so.oddEvenList(head);
  see_list(ans);
  return 0;
}
