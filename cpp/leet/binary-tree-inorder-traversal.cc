/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
 public:
  vector<int> inorderTraversal(TreeNode* root) {
    vector<int> result;
    inorderTraversal(root, result);
    return result;
  }

 private:
  void inorderTraversal(TreeNode* node, vector<int>& result) {
    if (!node) return;
    inorderTraversal(node->left, result);
    result.push_back(node->val);
    inorderTraversal(node->right, result);
  }
};
