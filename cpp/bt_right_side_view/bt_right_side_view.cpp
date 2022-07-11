// https://leetcode.com/problems/binary-tree-right-side-view/

#include <iostream>
#include <queue>
#include <my_utils.h>

class Solution {
public:
    vector<int> rightSideView(TreeNode* root) {
		queue<TreeNode*> bfs;
		vector<int> ans;

        if (root != nullptr)
            bfs.push(root);
		while (!bfs.empty()) {
			int size = bfs.size();

			for (int i=0; i<size; i++) {
				TreeNode* nextNode = bfs.front();
				bfs.pop();
				if (i == 0)
					ans.push_back(nextNode->val);
				
				if (nextNode->right != nullptr)
					bfs.push(nextNode->right);
				if (nextNode->left != nullptr)
					bfs.push(nextNode->left);
			}
		}
		return ans;        
    }
};


int main() {
	Solution solution;

}
