#include <iostream>
#include <my_utils.h>
#include <stack>
#include <vector>

// https://leetcode.com/problems/recover-binary-search-tree/

const int I32_MIN = -2147483648;

class Solution {
public:
    void recoverTree(TreeNode* root) {
		stack<TreeNode*> sortedOrder;
		TreeNode *misplacedNodes1, *misplacedNodes2, *lastNode;

		sortedOrder.push(root);
		lastNode = misplacedNodes1 = misplacedNodes2 = nullptr;
		while (!sortedOrder.empty()) {
			TreeNode *node = sortedOrder.top();

			if (node->left != nullptr) {
				sortedOrder.push(node->left);
				continue;
			}
			if (node->right != nullptr) {
				sortedOrder.push(node->right);
			}

			sortedOrder.pop();

			if (lastNode == nullptr) {
				lastNode = node;
				continue;
			}

			if (node->val < lastNode->val) {
				if (misplacedNodes1 == nullptr) {
					misplacedNodes1 = lastNode;
					misplacedNodes2 = node;
				}
				else {
					misplacedNodes2 = node;
					break;
				}
			}
			lastNode = node;
		}


    }

	void swap(TreeNode *node1, TreeNode *node2) {
		int tmp = node1->val;
		node1->val = node2->val;
		node2->val = tmp;
	}

};

int main() {
	Solution solution;
	vector<TreeNode*> testSet;
	TreeNode *leetCExample1, *leetCExample2, 
		*notRootSwitch, 
		*wrongPlacedChilds1, *wrongPlacedChilds2,
		*oneBranchTree1, *oneBranchTree2, *oneBranchTree3;

	leetCExample1 = new TreeNode(1, 
	new TreeNode(3, nullptr, new TreeNode(2)), 
	nullptr);

	leetCExample2 = new TreeNode(3, 
	new TreeNode(1), 
	new TreeNode(4, new TreeNode(2), nullptr));

	notRootSwitch = new TreeNode(3, 
	new TreeNode(1), 
	new TreeNode(4, new TreeNode(5), nullptr));

	wrongPlacedChilds1 = new TreeNode(2, new TreeNode(3), new TreeNode(1));

	wrongPlacedChilds2 = new TreeNode(5, 
	new TreeNode(4, 
		new TreeNode(3, new TreeNode(8), nullptr), 
		nullptr), 
	new TreeNode(6, 
		nullptr, 
		new TreeNode(7, nullptr, new TreeNode(2))));
	
	oneBranchTree1 = new TreeNode(3, 
	nullptr, 
	new TreeNode(2, nullptr, new TreeNode(1)));

	oneBranchTree2 = new TreeNode(0, 
	nullptr, 
	new TreeNode(3, 
		nullptr, 
		new TreeNode(2, nullptr, new TreeNode(1))));

	oneBranchTree3 = new TreeNode(1, 
	nullptr, 
	new TreeNode(3, nullptr, new TreeNode(2)));

	testSet.push_back(leetCExample1);
	testSet.push_back(leetCExample2);
	testSet.push_back(notRootSwitch);
	testSet.push_back(wrongPlacedChilds1);
	testSet.push_back(wrongPlacedChilds2);
	testSet.push_back(oneBranchTree1);
	testSet.push_back(oneBranchTree2);
	testSet.push_back(oneBranchTree3);


	cout << "Fixing BST which has exactly 2 misplaced nodes:" << endl;
	for (TreeNode *bst : testSet) {
		cout << "BST=" << NumberBTToString(bst) << endl;
		solution.recoverTree(bst);
		cout << "\t-> " << NumberBTToString(bst) << endl;
	}
}
