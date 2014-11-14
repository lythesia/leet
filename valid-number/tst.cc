#include <cstdio>
#include <iostream>
#include <regex>

using namespace std;

int main(int argc, const char *argv[])
{
  regex int_num("(\\+|-)?\\d+");
  if(argc >= 2) {
    cout << (regex_match(argv[1], int_num) ? "yes" : "no") << endl;
  }
  return 0;
}
