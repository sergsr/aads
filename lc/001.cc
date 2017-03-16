class Solution {
 public:
    vector<int> twoSum(const vector<int>& nums, int target) {
      unordered_map<int, int> nums_inverse;

      for (int i = 0; i < nums.size(); ++i) {
        nums_inverse.insert({ { nums[i], i } });
      }

      for (int i = 0; i < nums.size(); ++i) {
        auto lookup = nums_inverse.find(target - nums[i]);
        if (lookup != nums_inverse.end() && i != lookup->second) {
          return { i, lookup->second };
        }
      }
      return vector<int>();
    }
};
