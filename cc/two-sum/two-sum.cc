#include <cstdio>
#include <iostream>
#include <vector>
#include <unordered_map>
using namespace std;

/*
 * unordered_map use direct hash, so `[]` is O(1) on average
 */
typedef vector<int> vi;
typedef unordered_map<int, int> umii;
class Solution {
public:
  vi twoSum(vi &numbers, int target) {
    umii lst;
    vi res;
    for(int i=0; i<numbers.size(); i++) {
      int x = target - numbers[i];
      if(lst.find(x) != lst.end()) {
        res.push_back(lst[x]+1);
        res.push_back(i+1);
        return res;
      }
      lst[numbers[i]] = i;
    }
    return res;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  vi n = {3,2,4};
  vi res = so.twoSum(n, 6);
  cout << res.front() << ',' << res.back() << endl;
  return 0;
}
