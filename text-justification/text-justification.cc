#include <bits/stdc++.h>
using namespace std;

typedef vector<string> vs;
class Solution {
public:
  vs fullJustify(vs &ws, int width) {
    vs ans;
    int curlen = 0;
    vector<string*> curs;
    for(auto &s : ws) {
      int len = s.length();
      if(curlen + len + (int)curs.size() <= width) {
        curlen += len;
        curs.push_back(&s);
      }
      else {
        int nsp = curs.size() - 1;
        if(nsp) {
          int sp = (width - curlen) / nsp, rsp = (width - curlen) % nsp;
          string line;
          for(int i=0; i<(int)curs.size()-1; i++) line += *curs[i] + string(sp+(rsp-- > 0 ? 1:0), ' '); // evenly!
          line += *curs.back();
          ans.push_back(line);
        }
        else {  // one word line!
          string line(*curs[0]);
          ans.push_back(line + string(width-line.length(), ' '));
        }
        curlen = len;
        curs.clear(); curs.push_back(&s);
      }
    }
    string line;
    int left = width;
    for(int i=0; i<(int)curs.size()-1; i++) line += (*(curs[i]) + " "), left -= (curs[i]->length() + 1);
    string *last = curs.back();
    line += *last + string(left-last->length(), ' ');
    ans.push_back(line);
    return ans;
  }
};

int main(int argc, const char *argv[])
{
  // vs ws = {
  //   "This", "is", "an", "example", "of", "text", "justification.", "justification!"
  // };
  vs ws = {
    "Don't","go","around","saying","the","world","owes","you","a","living;","the","world","owes","you","nothing;","it","was","here","first."
  };
  int w = 30;
  Solution so;
  vs ans = so.fullJustify(ws, w);
  for(auto &s : ans) cout << "\"" << s << "\"" << endl;
  return 0;
}
