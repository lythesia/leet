#include <cstdio>
#include <cstdlib>
#include <cassert>
#include <iostream>
#include <queue>
#include <algorithm>
using namespace std;
struct ListNode {
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(NULL) {}
};
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};
typedef ListNode *link;
typedef TreeNode *node;
class Solution {
public:
  link before_mid(link head) {
    if(!head || !head->next) return head;
    link dummy = new ListNode(0);
    dummy->next = head;
    link pp = dummy, p = head, p2 = head;
    while(p2 && p2->next) pp = p, p = p->next, p2 = p2->next->next;
    assert(pp && pp->next && pp != dummy);
    delete dummy;
    return pp;
  }

  node sortedListToBST(link head) {
    if(!head) return NULL;
    if(!head->next) return new TreeNode(head->val);
    link p = before_mid(head), q = p->next->next;
    node root = new TreeNode(p->next->val);
    p->next = NULL;
    root->left = sortedListToBST(head);
    root->right = sortedListToBST(q);
    return root;
  }
};

void see_list(link head) {
  while(head) printf("%d -> ", head->val), head = head->next; puts("NULL");
}
void see_tree_level(node root) {
  if(!root) return;
  queue<node> Q;
  Q.push(root);
  while(!Q.empty()) {
    node top = Q.front();
    Q.pop();
    printf("%d ", top->val);
    if(top->left) Q.push(top->left);
    if(top->right) Q.push(top->right);
  }
  puts("");
}
#define N 1
int main(int argc, const char *argv[])
{
  int A[N];
  for(int i=0; i<N; i++) A[i] = i/*rand() % N*/;
  sort(A, A+N);
  int n = N;
  link p;
  link l1 = new ListNode(A[0]);
  p = l1;
  while(--n) p->next = new ListNode(A[N-n]), p = p->next;
  printf("A: "); see_list(l1);
  puts("--------");

  Solution so;
  node tree = so.sortedListToBST(l1);
  see_tree_level(tree);
  return 0;
}
