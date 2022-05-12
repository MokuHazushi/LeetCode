// https://leetcode.com/problems/closest-binary-search-tree-value/

#include <iostream>
#include <my_utils.h>
#include <math.h>
#include <vector>
#include <climits>

class Solution {
public:
    int closestValue(TreeNode* root, double target) {
		int integerTarget = (int)round(target);
		int closestVal = root->val;

		while (root != nullptr) {
			if (root->val == integerTarget)
				return root->val;	

			if (abs(closestVal - integerTarget) > abs(root->val - integerTarget))
				closestVal = root->val;

			if (integerTarget > root->val)
				root = root->right;
			else
				root = root->left;
		}
		return closestVal;
    }
};

struct Test {
	TreeNode *bt;
	double target;

	Test(TreeNode *bt, double target) : bt(bt), target(target) {}
};

int main() {
	Solution solution;
	vector<Test> testSet;

	TreeNode *bt1 = new TreeNode(4, 
		new TreeNode(2, new TreeNode(1), new TreeNode(3)), 
		new TreeNode(5));
	testSet.push_back(Test(bt1, 3.714));

	for (Test test : testSet) {
		cout << NumberBTToString(test.bt) << endl;
		cout << "\t-> " << solution.closestValue(test.bt, test.target) << endl;
	}

}
