class Solution {
 public:
  vector<int> getRow(int rowIndex) {
    vector<int> row;

    long n = rowIndex + 1;
    long elem = 1;
    for (long k = 0; k < n; ++k) {
      row.push_back(elem);
      elem *= (n - k - 1);
      elem /= (k + 1);
    }

    return row;
  }
};
