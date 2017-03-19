class Solution {
 public:
  static constexpr int firstLetter = (int)'a';
  static constexpr int lastLetter = (int)'z';
  static constexpr int alphabetSize = 26;

  char findTheDifference(string s, string t) {
    auto smallBucket = vector<int>(alphabetSize, 0);
    auto largeBucket = vector<int>(alphabetSize, 0);

    for (char& c : s) {
      ++smallBucket[((int)c) - firstLetter];
    }
    for (char& c : t) {
      ++largeBucket[((int)c) - firstLetter];
    }
    for (int i = 0; i < alphabetSize; ++i) {
      if (largeBucket[i] > smallBucket[i]) {
        return (char)(i + firstLetter);
      }
    }
    return '\0';
  }
};
