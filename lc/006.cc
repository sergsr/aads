class Solution {
 public:
  string convert(string s, int numRows) {
    if (numRows == 1) return s;

    vector<string> m;
    for (int i = 0; i < numRows; ++i) {
      m.push_back("");
    }

    int row = 0;
    bool forwards = true;
    for (int i = 0; i < s.size(); ++i) {
      m[row].push_back(s[i]);
      if (forwards) {
        if (row < numRows - 1) {
          row += 1;
        } else {
          forwards = false;
          row -= 1;
        }
      } else {
        if (row > 0) {
          row -= 1;
        } else {
          forwards = true;
          row += 1;
        }
      }
    }
    return rowOrder(m);
  }

 private:
  string rowOrder(const vector<string>& m) {
    string result;
    for (string row : m) {
      for (char c : row) {
        result.push_back(c);
      }
    }
    return result;
  }
};
