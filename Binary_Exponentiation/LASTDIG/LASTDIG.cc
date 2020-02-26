#include <cstdio>
#include <iostream>
#include <vector>

using namespace std;

vector<int> loop[10] = {{0}, {1}, {2, 4, 8, 6}, {3, 9, 7, 1}, {4, 6},
                        {5}, {6}, {7, 9, 3, 1}, {8, 4, 2, 6}, {9, 1}};

class Solution {
public:
  void solve() {
    int a, b;
    scanf("%d%d", &a, &b);
    if (b == 0)
      printf("1\n");
    else
      printf("%d\n", loop[a % 10][(b - 1) % loop[a % 10].size()]);
  }
};

int main() {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  scanf("%d", &t);
  while (t--) {
    Solution solution = Solution();
    solution.solve();
  }
}