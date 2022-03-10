#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  /*
   * Manachar(http://en.wikipedia.org/wiki/Longest_palindromic_substring)
   * 12212321 join with # into
   * S: # 1 # 2 # 2 # 1 # 2 # 3 # 2 # 1 #
   * P: 1 2 1 2 5 2 1 4 1 2 1 6 1 2 1 2 1 P[i]: center with S[i], the longest length of a palidrome extend to left
   * or right(include S[i]), and now P[i] - 1 the length of pali
   * say: mx = id + P[id]
   *
   *    mx'   j     id      i   mx        <- i and j are symmetirc to id, so are mx and mx'
   * ----+----+------+------+----+--------
   *
   * 1. P[i] = P[j] if mx - i > P[j]: pali center with S[j] is included within S[id], so is S[i] (since symmetirc,
   *    S[i] is also pali, and S[i] at MOST extend to mx)
   * 2. P[i] = mx - i elsif mx - i <= P[j]: pali center with S[j] maybe not included with S[id], due to symmetirc
   *    S[i] as pali, at LEAST extend to mx, or say P[i] >= mx - i, but we do not know whether after mx is also
   *    pali, because only thing we sure is [mx', mx] is pali, so we take the least right is [2i-mx, mx] is a
   *    valid pali.
   * 3. P[i] = 1 if mx <= i which is trivial
   *
   * shorten as:
   *
   *  if mx > i
   *    if mx - i > P[j]: P[i] = P[j]
   *    else mx - i < P[j]: P[i] = mx - i
   *  else: P[i] = 1
   *
   *  as:
   *
   *  if mx > i: P[i] = min(P[j], mx - i)
   *  else P[i] = 1
   */
  string longestPalindrome(string s) {
    int len = s.length(), p[2002] = {0};
    for(int i=0, id=0, mx=0; i<=2*(len-1); i++) {
      /* arrange as: a0 # a1 # .. # an
       * char is at 2k, # is at 2k+1
       *
       * if #: length of d not include #
       * else: length of d include current char
       */
      /*
       * mx - i, len -> k
       * if 2k+1:   one of i and mx is #, think it when id is # or not, both is correct with following `while` :)
       * else(2k):  both # or both char
       */
      int d = i < mx ? min(p[id + id - i], (mx - i) / 2) : 0; // count real chars in s stead of the one with #'s
      // if i # : i/2 => real i in s; (i+1)/2 => real i+1 in s
      // else   : both => real i
      int a = i / 2 - d, b = (i + 1) / 2 + d; // [a .. i .. b]
      while(0 <= a && b < len && s[a] == s[b]) --a, ++b, ++d; // anyway, d will be handled correctly here
      p[i] = d;
      // update mx also id, 2*b-1 since last b is invalid for pali
      if(mx < 2*b-1) id = i, mx = 2*b-1;
    }
    int mlen = 0, index = 0, clen = 0;
    for(int i=0; i<=2*(len-1); i++) {
      if(i & 1) { // #
        clen = p[i]*2;
        if(clen > mlen) mlen = clen, index = (i - p[i]*2 + 1) / 2;
      }
      else {
        clen = p[i]*2 - 1;
        if(clen > mlen) mlen = clen, index = (i - p[i]*2 + 2) / 2;
      }
    }
    return s.substr(index, mlen);
  }
};

int main(int argc, const char *argv[])
{
  vector<string> vs = {
    "12212321",
    "babcbabcbaccba",
    "abaaba",
    "abababa",
    "abcbabcbabcba",
    "forgeeksskeegfor",
    "caba",
    "abacdfgdcaba",
    "abacdfgdcabba",
    "abacdedcaba",
    "bb",
  };
  Solution so;
  for(auto &s : vs)
    cout << s << ": " << so.longestPalindrome(s) << endl;
  return 0;
}
