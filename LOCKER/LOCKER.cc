#include <cstdio>
#include <iostream>
#define MOD 1000000007

using namespace std;
typedef long long ll;

ll fexp(ll x, ll y) {
  ll ans = 1;
  while (y > 0) {
    if (y & 1)
      ans = ans * x % MOD;
    x = x * x % MOD;
    y >>= 1;
  }
  return ans;
}

class Solution {
public:
  void solve() {
    ll n;
    scanf("%lld", &n);
    if (n <= 4)
      printf("%lld\n", n);
    else {
      ll parts = n / 3;
      ll ans = fexp(3, parts);
      if (n % 3 == 2)
        ans = ans * 2 % MOD;
      else if (n % 3 == 1)
        ans = fexp(3, parts - 1) * 4 % MOD;
      printf("%lld\n", ans);
    }
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