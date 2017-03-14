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
    TreeNode* invertTree(TreeNode* root) {
        if(!root) return NULL;

        TreeNode* result(new TreeNode(root->val));
        result->left = invertTree(root->right);
        result->right = invertTree(root->left);

        return result;
    }
};
