#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

class TrieNode {
public:
  TrieNode(bool w = false) : isw(w) {}
  bool isw;
  TrieNode *child[26] = {NULL};
};
typedef TrieNode *node;

class Trie {
public:
  Trie() {
    root = new TrieNode();
  }

  void insert(string s) {
    node p = root;
    for(size_t i=0; i<s.length(); i++) {
      size_t pos = s[i] - 'a';
      if(!p->child[pos]) p->child[pos] = new TrieNode();
      p = p->child[pos];
    }
    p->isw = true;
  }

  bool search(string k) {
    node p = dofind(k);
    return p && p->isw;
  }

  bool startsWith(string p) {
    return dofind(p) != NULL;
  }

  node dofind(string k) {
    node p = root;
    for(size_t i=0; i<k.length() && p; i++) p = p->child[k[i]-'a'];
    return p;
  }

private:
  node root;
};

int main(int argc, const char *argv[])
{
  Trie t;
  vs v = {"and", "as", "at", "cn", "com"};
  for(auto &s : v) t.insert(s);
  cout << boolalpha;
  cout << t.search("cn") << endl;
  cout << t.search("fuck") << endl;
  cout << t.startsWith("an") << endl;
  cout << t.startsWith("foo") << endl;
  return 0;
}
