// https://leetcode.com/problems/find-a-corresponding-node-of-a-binary-tree-in-a-clone-of-that-tree/

#include <iostream>
#include <my_utils.h>
#include <queue>
#include <vector>

class Solution {
public:
    TreeNode* getTargetCopy(TreeNode* original, TreeNode* cloned, TreeNode* target) {
		queue<TreeNode*> bfs;

		bfs.push(cloned);
		while (bfs.size() != 0) {
			TreeNode *element = bfs.front();
			bfs.pop();

			if (element->val = target->val)
				return element;
			
			if (element->left != nullptr)
				bfs.push(element->left);
			if (element->right != nullptr)
				bfs.push(element->right);
		}
		return nullptr;
    }
};

int main() {
	Solution solution;
}
