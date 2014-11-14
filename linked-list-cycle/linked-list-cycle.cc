#include <cstdio>
#include <iostream>
using namespace std;

struct ListNode {
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(NULL) {}
};
typedef ListNode *link;
class Solution {
public:
  bool hasCycle(ListNode *head) {
    if(!head || !head->next) return false;
    link fast = head, slow = head;
    while(fast && fast->next) {
      fast = fast->next->next;
      slow = slow->next;
      if(fast == slow) return true;
    }
    return false;
  }
};

int main(int argc, const char *argv[])
{
  
  return 0;
}
