#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

static string d[] = {"", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
                    "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"};
static string d2[] = {"", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"}; 
static string d3[] = {"", "Thousand", "Million", "Billion"};
class Solution {
public:
  string chunk(int n) {
    if(n < 20) return d[n];
    else if(n < 100) return n % 10 ? (d2[n / 10] + " " + chunk(n % 10)) : d2[n / 10];
    else return n % 100 ? (d[n / 100] + " Hundred " + chunk(n % 100)) : (d[n / 100] + " Hundred");
  }
  string numberToWords(int n) {
    int ch = 0, d = 0;
    string ans;
    while(n) {
      if((d = n % 1000)) {
        string c = chunk(d);
        if(ans.empty()) ans.swap(c);
        else ans = c + " " + ans;
      }
      n /= 1000, ch++;
      if(n % 1000) {
        if(ans.empty()) ans = d3[ch];
        else ans = d3[ch] + " " + ans;
      }
    }
    return ans.empty() ? "Zero" : ans;
  }
};

int main(int argc, const char *argv[])
{
  Solution so;
  cout << so.numberToWords(1000060) << endl;
  cout << so.numberToWords(10000100) << endl;
  cout << so.numberToWords(0) << endl;
  return 0;
}
