class Solution {
 public:
  static constexpr size_t UNIQUE_CHAR_COUNT = 1 << 8 * sizeof(char);

  int lengthOfLongestSubstring(string s) {
    int length = s.length();

    if (length < 2) return length;

    int left = 0;
    int right = 1;
    int result = 1;

    bitset<UNIQUE_CHAR_COUNT> encountered;
    encountered.set(s[left]);
    while (right < length) {
      // until we find a repeated element, go as far right as possible
      while (right < length && !encountered[s[right]]) {
        encountered.set(s[right++]);
      }
      result = max(result, right - left);
      if (right == length) return result;

      // until all unique again, move left end of window to the right
      while (s[left] != s[right]) {
        encountered.reset(s[left++]);
      }

      if (left < right) ++left;
      ++right;
    }

    return result;
  }
};
