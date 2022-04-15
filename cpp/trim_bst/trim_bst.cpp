#include <iostream>
#include <my_utils.h>
#include <vector>

struct Test {
	int low, high;
	TreeNode *bst;

	Test(int low, int high, TreeNode *bst) : low(low), high(high), bst(bst) {}
};

class Solution {
public:
    TreeNode* trimBST(TreeNode* root, int low, int high) {
		if (root == nullptr)
			return nullptr;

		root->left = trimBST(root->left, low, high);
		root->right = trimBST(root->right, low, high);

		if (root->val < low)
			return root->right;

		if (root->val > high)
			return root->left;

		return root;		
    }
};

int main() {
	Solution solution;
	TreeNode *leetCExample1, *leetCExample2;
	vector<Test> testSet;

	leetCExample1 = new TreeNode(1, new TreeNode(0), new TreeNode(2));
	leetCExample2 = new TreeNode(3, 
	new TreeNode(0, 
		nullptr, 
		new TreeNode(2, 
			new TreeNode(1, nullptr, nullptr),
			nullptr
		)
	),
	new TreeNode(4));

	testSet.push_back(Test(1, 2, leetCExample1));
	testSet.push_back(Test(1, 3, leetCExample2));

	cout << "Triming BST in range";
	for (Test test : testSet) {
		cout << "Trim range [" << test.low << ", " << test.high << "] ";
		cout << "in BST=" << NumberBTToString(test.bst) << endl;
		test.bst = solution.trimBST(test.bst, test.low, test.high);
		cout << "\t-> " << NumberBTToString(test.bst) << endl;
	}
	
}
