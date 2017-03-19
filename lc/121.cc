class Solution {
 public:
  int maxProfit(const vector<int>& prices) {
    int currentMin = INT_MAX;
    int currentBest = 0;
    for (const int& p : prices) {
      if (p < currentMin) {
        currentMin = p;
      } else {
        currentBest = max(p - currentMin, currentBest);
      }
    }
    return currentBest;
  }
};
