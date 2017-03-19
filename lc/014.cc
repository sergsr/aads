class Solution {
 public:
  string longestCommonPrefix(const vector<string> &strs) {
    if (strs.empty()) return "";
    if (strs.size() == 1) return strs[0];

    int lastIndex = strs[0].size();
    for (const string &s : strs) {
      for (int i = 0; i <= lastIndex; ++i) {
        if (strs[0][i] != s[i]) {
          lastIndex = i;
          break;
        }
      }
    }
    return strs[0].substr(0, lastIndex);
  }
};
