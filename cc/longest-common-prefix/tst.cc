#include <iostream>
#include <cstdio>
#include <climits>
#include <string>

using namespace std;

int main(int argc, const char *argv[])
{
  string s = "";
  //cout << s.at(0) << endl; // err
  //cout << s.length() << endl;
  char buf[128];
  //fgets(buf, 128, stdin); // reads `\n`, so last is it
  //printf("%s\n", buf);
  while(fgets(buf, 128, stdin)) {
    string ss(buf);
    cout << ss.length() << ',' << ss.at(ss.length() - 1) << ':' ;
    printf("%s", buf);
  }

  cout << INT_MAX << endl;
  return 0;
}
