#include <iostream>
#include <my_utils.h>
#include <stack>
#include <vector>

// https://leetcode.com/problems/recover-binary-search-tree/

class Solution {
public:
    void recoverTree(TreeNode* root) {
		stack<TreeNode*> sortedOrder;
		TreeNode *misplacedNodes1, *misplacedNodes2, *current, *lastNode, *successor;

		sortedOrder.push(root);
		current = root;
		misplacedNodes1 = misplacedNodes2 = lastNode = nullptr;
		while (!sortedOrder.empty()) {
			while (current != nullptr) {
				current = current->left;
				if (current != nullptr)
					sortedOrder.push(current);
			}

			TreeNode *node = sortedOrder.top();
			sortedOrder.pop();
			current = node->right;
			
			if (current != nullptr)
				sortedOrder.push(current);

			if (lastNode == nullptr) {
				lastNode = node;
				continue;
			}

			if (lastNode->val > node->val) {
				if (misplacedNodes1 == nullptr) {
					misplacedNodes1 = lastNode;
					successor = node;
				}
				else {
					misplacedNodes2 = node;
					break;
				}
			}
			lastNode = node;
		}

		if (misplacedNodes2 == nullptr)
			misplacedNodes2 = successor;

		swap(misplacedNodes1, misplacedNodes2);

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
