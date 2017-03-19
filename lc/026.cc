class Solution {
 public:
  int removeDuplicates(vector<int> &nums) {
    if (nums.empty()) return 0;

    int length = 1;
    int previous = nums[0];
    for (int i = 1; i < nums.size(); ++i) {
      if (nums[i] != previous) {
        nums[length] = nums[i];
        ++length;
      }
      previous = nums[i];
    }
    return length;
  }
};
