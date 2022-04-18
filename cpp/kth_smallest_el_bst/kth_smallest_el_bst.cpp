#include <iostream>
#include <my_utils.h>

/* https://leetcode.com/problems/kth-smallest-element-in-a-bst/ */

struct Test {
	int k;
	TreeNode* bst;
	Test(int k, TreeNode* bst) : k(k), bst(bst) {}
};

struct Accumulator {
	int k, ans;
	Accumulator(int k, int ans) : k(k), ans(ans) {}
};

class Solution {
public:
    int kthSmallest(TreeNode* root, int k) {
		return kthSmallest_rec(root, Accumulator(k, -1)).ans;
	}

	Accumulator kthSmallest_rec(TreeNode* node, Accumulator acc) {
		if (node == nullptr)
			return acc;
		acc = kthSmallest_rec(node->left, acc);
		acc.k--;
		if (acc.k == 0) {
			acc.ans = node->val;
			return acc;
		}
		return kthSmallest_rec(node->right, acc);
	}

};

int main() {
	Solution solution;
	TreeNode *leetCExample1, *leetCExample2, *myExample;
	vector<Test> testSet;

	leetCExample1 = new TreeNode(3, 
	new TreeNode(1, nullptr, new TreeNode(2)), 
	new TreeNode(4));

	leetCExample2 = new TreeNode(5, 
	new TreeNode(3, 
		new TreeNode(2, new TreeNode(1), nullptr), 
		new TreeNode(4)), 
	new TreeNode(6));

	myExample = new TreeNode(60, 
	new TreeNode(30, 
		new TreeNode(10), 
		new TreeNode(50, new TreeNode(40), nullptr)), 
	new TreeNode(70));

	testSet.push_back(Test(1, leetCExample1));
	testSet.push_back(Test(3, leetCExample2));
	testSet.push_back(Test(1, leetCExample2));
	testSet.push_back(Test(6, leetCExample2));
	testSet.push_back(Test(3, myExample));

	cout << "Find kth smallest element in BST:" << endl;
	for (Test test : testSet) {
		cout << "k=" << test.k << ", BST=" << NumberBTToString(test.bst) << endl;
		cout << "\t-> " << solution.kthSmallest(test.bst, test.k) << endl;
	}
}
