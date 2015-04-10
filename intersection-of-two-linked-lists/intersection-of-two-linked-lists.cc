#include <bits/stdc++.h>
using namespace std;

struct ListNode {
  int val;
  ListNode *next;
  ListNode(int x): val(x), next(NULL) {}
};
typedef ListNode *node;
class Solution {
public:
  int len(node h) {
    int ans = 0;
    while(h) ans++, h = h->next;
    return ans;
  }
  node getIntersectionNode(node ha, node hb) {
    if(!ha || !hb) return NULL;
    int lena = len(ha), lenb = len(hb), diff = abs(lena - lenb);
    while(diff--) lena > lenb ? ha = ha->next : hb = hb->next;
    while(ha && hb && ha != hb) ha = ha->next, hb = hb->next;
    return (ha && hb) ? ha : NULL;
  }
};
