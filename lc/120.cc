class Solution {
 public:
  int minimumTotal(vector<vector<int>>& triangle) {
    if (triangle.empty()) return 0;

    vector<int> accum(triangle.back());
    for (int i = triangle.size() - 2; i >= 0; --i) {
      for (int j = 0; j < triangle[i].size(); ++j) {
        accum[j] = triangle[i][j] + min(accum[j], accum[j + 1]);
      }
    }
    return accum[0];
  }
};
