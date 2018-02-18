class Solution {
 public:
  int removeElement(vector<int>& nums, int val) {
    int length = nums.size();
    for (int i = length - 1; i >= 0; --i) {
      if (nums[i] == val) {
        swap(nums[i], nums[length - 1]);
        --length;
      }
    }
    return length;
  }
};
