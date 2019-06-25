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
typedef ListNode *node;
class Solution {
public:
  node rev(node head) {
    if(!head || !head->next) return head;
    node prev = head, curr = head->next;
    while(curr) {
      node tmp = curr->next;
      curr->next = prev, prev = curr, curr = tmp;
    }
    head->next = NULL;
    return prev;
  }

  bool isPalindrome(node head) {
    if(!head || !head->next) return true;
    int n = 0;
    for(node p=head; p; p=p->next) n++;
    node mid = head, p = head;
    while(p && p->next) mid = mid->next, p = p->next->next;
    node r = rev((n & 1) ? mid->next : mid);
    for(node i = head, j = r;
        i != mid && j;
        i = i->next, j = j->next) if(i->val != j->val) return false;
    return true;
  }
};

#define N 4
void see_list(node head) {
  while(head) {
    cout << head->val << ' ';
    head = head->next;
  }
  cout << "NULL" << endl;
}
int main(int argc, const char *argv[])
{
  int A[] = {1,2,2,3}, n = N;
  node head = new ListNode(A[0]), p = head;
  while(--n) p->next = new ListNode(A[N-n]), p = p->next;
  see_list(head);
  cout << "----" << endl;

  Solution so;
  cout << boolalpha << so.isPalindrome(head) << endl;
  return 0;
}
