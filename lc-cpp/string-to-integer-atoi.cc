class Solution {
 public:
  int myAtoi(string str) {
    int result = 0;
    bool hadNumber = false;
    bool isNeg = false;
    bool hadSign = false;
    for (char c : str) {
      // non numeric character
      if (c < '0' || c > '9') {
        // already got a number and not whitespace or first minus
        if (hadNumber || hadSign) break;
        if (c == ' ') continue;
        if (c == '+' && !hadSign) {
          hadSign = true;
        } else if (c == '-' && !hadSign) {
          isNeg = true;
          hadSign = true;
        } else {
          break;
        }
      } else {
        if (INT_MAX / 10 < result) {
          return isNeg ? INT_MIN : INT_MAX;
        }
        result *= 10;
        if (INT_MAX - (c - '0') < result) {
          return isNeg ? INT_MIN : INT_MAX;
        }
        result += c - '0';
        hadNumber = true;
      }
    }
    return isNeg ? -1 * result : result;
  }
};
