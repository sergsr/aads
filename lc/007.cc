class Solution {
 public:
  int reverse(int x) {
    bool neg = x < 0;
    if (neg) x *= -1;

    int result = 0;
    while (x > 0) {
      if (INT_MAX / 10 < result) return 0;
      result *= 10;
      if (INT_MAX - (x % 10) < result) return 0;
      result += x % 10;
      x /= 10;
    }
    return neg ? -1 * result : result;
  }
};
