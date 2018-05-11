class Solution {
 public:
  int maxArea(vector<int>& height) {
    int result = 0;
    int left = 0;
    int right = height.size() - 1;
    while (left < right) {
      int area = (right - left) * std::min(height[left], height[right]);
      if (height[left] < height[right])
        ++left;
      else
        --right;
      result = std::max(result, area);
    }
    return result;
  }
};
