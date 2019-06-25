#include <bits/stdc++.h>
using namespace std;

struct ListNode {
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(NULL) {}
};
typedef ListNode *node;
class Solution {
public:
  node reverseList(node head) {
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
};
