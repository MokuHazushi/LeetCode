// https://leetcode.com/problems/maximum-depth-of-binary-tree/

#include <iostream>
#include <my_utils.h>
#include <vector>
#include <queue>

class Solution {
public:
    int maxDepth(TreeNode* root) {
		if (root == nullptr)
			return 0;
			
		queue<TreeNode*> queue;
		int maxDepth = 0;

		queue.push(root);
		while (!queue.empty()) {
			maxDepth++;
			int size = queue.size();
			for (int i=0; i<size; i++) {
				TreeNode *node = queue.front();
				queue.pop();

				if (node->left != nullptr)
					queue.push(node->left);
				if (node->right != nullptr)
					queue.push(node->right);
			}		
		}
		return maxDepth;        
    }
};


int main() {
	Solution solution;
	vector<TreeNode*> testSet;

	testSet.push_back(new TreeNode(3, 
		new TreeNode(9), 
		new TreeNode(20, new TreeNode(15), new TreeNode(7))));

	for (TreeNode *test : testSet) {
		cout << "BT=" << NumberBTToString(test) << endl;
		cout << "\t-> " << solution.maxDepth(test) << endl;
	}
}
