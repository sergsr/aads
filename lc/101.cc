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
    bool skewEquals(TreeNode* a, TreeNode* b) {
        if ((bool) a != (bool) b) return false;
        if (!a) return true;
        if (a->val != b->val) return false;
        return skewEquals(a->left, b->right) && skewEquals(a->right, b->left);
    }

public:
    bool isSymmetric(TreeNode* root) {
        if (!root) return true;
        return skewEquals(root->left, root->right);
    }
};
