#include <bits/stdc++.h>
using namespace std;

#define inject(x) { cerr << "Function: " << __FUNCTION__ << ", Line: " << __LINE__ << ", " << #x << ": " << (x) << endl; }

typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<string> vs;
typedef vector<vs> vvs;

typedef TreeNode *node;
class Codec {
public:
  string serialize(node root) {
    ostringstream out;
    ser(root, out);
    return out.str();
  }
  node deserialize(string data) {
    istringstream in(data);
    return deserialize(in);
  }

  void ser(node root, ostringstream &out) {
    if(root) {
      out << root->val << ' ';
      ser(root->left, out);
      ser(root->right, out);
    }
    else out << "# ";
  }

  node deser(istringstream &in) {
    string s;
    in >> s;
    if(s == "#") return nullptr;
    else {
      node root = new TreeNode(stoi(s));
      root->left = deser(in);
      root->right = deser(in);
      return root;
    }
  }
};

// Your Codec object will be instantiated and called as such:
// Codec codec;
// codec.deserialize(codec.serialize(root));
