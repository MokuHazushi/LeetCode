#include <iostream>
#include <my_utils.h>
#include <vector>

class Solution {
public:
    TreeNode* increasingBST(TreeNode* root) {
		TreeNode *ans = new TreeNode(-1);
		TreeNode *iter = ans;

		increasingBT_rec(root, &iter);
		return ans->right;        
    }

	void increasingBT_rec(TreeNode* node, TreeNode **acc) {
		if (node == nullptr)
			return;

		increasingBT_rec(node->left, acc);
		(*acc)->right = new TreeNode(node->val);
		(*acc) = (*acc)->right;
		increasingBT_rec(node->right, acc);
	}
};

int main() {
	Solution solution;
	TreeNode *leetCExample;
	vector<TreeNode*> testSet;

	leetCExample = new TreeNode(5, 
	new TreeNode(3, 
		new TreeNode(2, new TreeNode(1), nullptr), 
		new TreeNode(4)), 
	new TreeNode(6, 
		nullptr, 
		new TreeNode(8, new TreeNode(7), new TreeNode(9))));

	testSet.push_back(leetCExample);

	cout << "Rearrange BST in-order:" << endl;
	for (TreeNode *test : testSet) {
		cout << "BST=" << NumberBTToString(test) << endl;
		cout << "\t-> " << NumberBTToString(solution.increasingBST(test)) << endl;
	}
}
