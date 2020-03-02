#include <cstdio>
#include <iostream>
#define MOD 10000007

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

int main() {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, k;
  while (true) {
    scanf("%d%d", &n, &k);
    if (!n)
      break;
    ll ans = 2 * fexp(n - 1, k) + fexp(n, k);
    ans += 2 * fexp(n - 1, n - 1) + fexp(n, n);
    printf("%lld\n", ans % MOD);
  }
}