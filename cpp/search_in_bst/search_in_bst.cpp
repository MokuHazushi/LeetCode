#include <iostream>
#include <my_utils.h>

class Solution {
public:
    TreeNode* searchBST(TreeNode* root, int val) {
        while (root != nullptr && root->val != val)
			root = val < root->val ? root->left : root->right;
		return root;
    }
};

int main() {
	Solution solution;
}