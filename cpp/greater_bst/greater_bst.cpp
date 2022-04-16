#include <iostream>
#include <my_utils.h>
#include <vector>

class Solution {
public:
    TreeNode* convertBST(TreeNode* root) {
		TreeNode *iter = root;
		convertBST_rec(iter, 0);
		return root;        
    }

	int convertBST_rec(TreeNode *node, int max) {
		if (node == nullptr)
			return max;

		max = convertBST_rec(node->right, max);
		node->val += max;
		max = node->val;
		return convertBST_rec(node->left, max);
	}
};

int main() {
	Solution solution;
	TreeNode *leetCExample, *emptyTree, *oneNodeTree;
	vector<TreeNode*> testSet;

	leetCExample = new TreeNode(4, 
	new TreeNode(1, 
		new TreeNode(0), 
		new TreeNode(2, nullptr, new TreeNode(3))), 
	new TreeNode(6, 
		new TreeNode(5), 
		new TreeNode(7, nullptr, new TreeNode(8))));
	emptyTree = nullptr;
	oneNodeTree = new TreeNode(0);

	testSet.push_back(leetCExample);
	testSet.push_back(emptyTree);
	testSet.push_back(oneNodeTree);

	cout << "Convert BST to greater tree:" << endl;
	for (TreeNode *test : testSet) {
		cout << "BST=" << NumberBTToString(test) << endl;
		solution.convertBST(test);
		cout << "\t->" << NumberBTToString(test) << endl;
	}
}
