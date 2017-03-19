class Solution {
  vector<int> pascalRow(int n) {
    vector<int> row;

    int elem = 1;
    for (int k = 0; k < n; ++k) {
      row.push_back(elem);
      elem *= (n - k - 1);
      elem /= (k + 1);
    }

    return row;
  }

 public:
  vector<vector<int>> generate(int numRows) {
    vector<vector<int>> rows;

    for (int i = 1; i <= numRows; ++i) {
      rows.push_back(pascalRow(i));
    }
    return rows;
  }
};
