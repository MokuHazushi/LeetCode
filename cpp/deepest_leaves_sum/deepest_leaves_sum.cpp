// https://leetcode.com/problems/deepest-leaves-sum/

#include <iostream>
#include <my_utils.h>
#include <vector>
#include <queue>

class Solution {
public:
    int deepestLeavesSum(TreeNode* root) {
		int sum = 0;
		queue<TreeNode*> bfs;

		bfs.push(root);
		while (!bfs.empty()) {
			int queueSize = bfs.size();
			sum = 0;

			for (int i=0; i<queueSize; i++) {
				TreeNode *front = bfs.front();
				bfs.pop();
				sum += front->val;
				if (front->left != nullptr)
					bfs.push(front->left);
				if (front->right != nullptr)
					bfs.push(front->right);
			}
		}
		return sum;        
    }
};

int main() {
	Solution solution;
	vector<TreeNode*> testSet;

	testSet.push_back(new TreeNode(1,
		new TreeNode(2, 
			new TreeNode(4, new TreeNode(7), nullptr), 
			new TreeNode(5)),
		new TreeNode(3, 
			nullptr,
			new TreeNode(6, nullptr, new TreeNode(8)))));
	
	for (TreeNode* test : testSet) {
		cout << "Tree= " << NumberBTToString(test) << endl;
		cout << "\t-> " << solution.deepestLeavesSum(test) << endl;
	}
}