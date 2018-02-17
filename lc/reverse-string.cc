class Solution {
 public:
  string reverseString(string s) {
    std::string result = std::string();
    for (int i = s.length() - 1; i >= 0; --i) {
      result.push_back(s[i]);
    }
    return result;
  }
};
