#include <cstdio>
#include <iostream>
#include <cstring>
#include <vector>
#include <utility>
#include <iterator>
using namespace std;

typedef vector<char> vc;
typedef vector<vc> vvc;
typedef vector<bool> vb;

void pboard(vvc &board) {
  for(auto &v : board) {
    copy(v.begin(), v.end(), ostream_iterator<char>(cout, " ")); cout << endl;
  }
}

/*
 * header: c1 c2 c3 .. cn
 *          |  |  |     |
 *          v  v  v     v
 *
 * M: n x n x n: possible nodes
 * N: n x n x 4, 4 constraints:
 *    1. 1 - nxn: 每个格子一个数
 *    2. nxn+1 - 2xnxn: 每列包含 1-n
 *    3. 2xnxn+1 - 3xnxn: 每行包含 1-n
 *    4. 3xnxn+1 - 4xnxn: 每宫包含 1-n
 */
template <int M, int N>
struct DLX {
  const static int MAX = (M + 1) + // header ptr + header
                  (N + 1);  // nodes
  int m, n, size;
  int U[MAX], D[MAX], L[MAX], R[MAX], Row[MAX], Col[MAX],
      H[M+1], S[N+1]; // row ptr; column counter
  int ansd, ans[M+1]; 

  void init(int mm, int nn) {
    m = mm, n = nn;
    // 0 is header ptr, 1 - n is header: c1 - cn
    for(int i=0; i<=n; i++) {
      S[i] = 0; // column counter = 0
      U[i] = D[i] = i;  // headers' up/down to self
      L[i] = i-1, R[i] = i+1; // headers' left/right to neighbour
    }
    R[n] = 0, L[0] = n; // headers' head/tail
    size = n;
    for(int i=1; i<=m; i++) H[i] = -1; // row ptr as NULL
  }

  void link(int r, int c) {
    ++S[Col[++size] = c]; // new node's column = c; ++header count of column c
    Row[size] = r;        // new node's row = r
    D[size] = D[c], U[D[c]] = size, U[size] = c, D[c] = size; // insert after c(c)'s header
    if(H[r] >= 0) R[size] = R[H[r]], L[R[H[r]]] = size, L[size] = H[r], R[H[r]] = size;
    else H[r] = L[size] = R[size] = size; // this row is empty
  }

  void remove(int c) {
    L[R[c]] = L[c], R[L[c]] = R[c]; // remove from header
    for(int i=D[c]; i!=c; i=D[i]) { // from column c's up(2nd) to bottom
      for(int j=R[i]; j!=i; j=R[j]) { // from i's left(2nd) to right
        U[D[j]] = U[j], D[U[j]] = D[j], // remove from verticle
        --S[Col[j]];
      }
    }
  }

  void resume(int c) { // reverse against remove
    for(int i=U[c]; i!=c; i=U[i]) {
      for(int j=L[i]; j!=i; j=L[j]) {
        ++S[Col[U[D[j]] = D[U[j]] = j]];
      }
    }
    L[R[c]] = R[L[c]] = c;
  }

  bool dance(int d) { // d for depth, or say rows we picked, at last nxn rows
    if(R[0] == 0) return ansd = d, true;  // header_ptr.right == header
    int c = R[0];
    for(int i=R[0]; i!=0; i=R[i]) {
      if(S[i] < S[c]) c = i;  // pick least column
    }
    remove(c);
    for(int i=D[c]; i!=c; i=D[i]) {
      ans[d] = Row[i]; // pick row of i-th node
      for(int j=R[i]; j!=i; j=R[j]) remove(Col[j]);
      if(dance(d+1)) return true;
      for(int j=L[i]; j!=i; j=L[j]) resume(Col[j]);
    }
    resume(c);
    return false;
  }
};

const int NN = 9;
/*
 * c1: 1 number in NxN
 * c2: 1 number k in col
 * c3: 1 number k in row
 * c4: 1 number k in block
 */
void place(int &r, int &c1, int &c2, int &c3, int &c4, int i, int j, int k) {
  r = (i*NN+j)*NN + k;
  c1 = i*NN+j+1;
  c2 = NN*NN + i*NN + k;
  c3 = NN*NN*2 + j*NN + k;
  c4 = NN*NN*3 + ((i/3)*3 + (j/3))*NN + k;
}

template<int M, int N>
void out(DLX<M, N> &dlx, vvc &b) {
  for(int i=0; i<dlx.ansd; i++) {
    int r = i/NN, c = i%NN;
    if(b[r][c] == '.') b[r][c] = (dlx.ans[i] - 1) % 9 + '1';
  }
}

class Solution {
public:
  void solveSudoku(vvc &board) {
    DLX<NN*NN*NN, 4*NN*NN> dlx;
    dlx.init(NN*NN*NN, 4*NN*NN);
    int r, c1, c2, c3, c4;
    for(int i=0; i<NN; i++) {
      for(int j=0; j<NN; j++) {
        for(int k=1; k<=NN; k++) {
          if(board[i][j] == '.' || board[i][j] == '0'+k) {
            place(r, c1, c2, c3, c4, i, j, k);
            dlx.link(r, c1);
            dlx.link(r, c2);
            dlx.link(r, c3);
            dlx.link(r, c4);
          }
        }
      }
    }
    dlx.dance(0);
    out(dlx, board);
  }
};

int main(int argc, const char *argv[])
{
  vvc board(9, vc(9));
  Solution so;
  for(int i=0; i<9; i++) {
    for(int j=0; j<9; j++) {
      int x = 0;
      scanf("%d", &x);
      board[i][j] = !x ? '.' : x+'0';
    }
  }
  so.solveSudoku(board);
  pboard(board);
  return 0;
}
