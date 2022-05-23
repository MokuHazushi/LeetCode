#include <iostream>
#include <my_utils.h>
#include <stack>
#include <vector>
#include <string>

// https://leetcode.com/problems/binary-search-tree-iterator/

class BSTIterator {
public:
    BSTIterator(TreeNode* root) {
		iter.push(root);
		current = root->left;
    }
    
    int next() {
		TreeNode *node;

		while (current != nullptr) {
			iter.push(current);
			current = current->left;
		}

		node = iter.top();
		iter.pop();
		current = node->right;
		return node->val;
    }
    
    bool hasNext() {
        return !iter.empty() || current != nullptr;
    }

private:
	stack<TreeNode*> iter;
	TreeNode *current;
};


int main() {
	vector<TreeNode*> testSet;
	TreeNode *leetCExample;

	leetCExample = new TreeNode(7, 
	new TreeNode(3), 
	new TreeNode(15, new TreeNode(9), new TreeNode(20)));

	testSet.push_back(leetCExample);

	cout << "Implement a BST in-order iterator:" << endl;
	for (TreeNode *test : testSet) {
		BSTIterator iterator(test);
		string s("[");
		cout << "BST=" << NumberBTToString(test) << endl;
		while (iterator.hasNext()) {
			s.append(to_string(iterator.next()));
			s.append(", ");
		}
		s.erase(s.end()-2, s.end());
    	s.append("]");
		cout << "\t-> " << s << endl;
	}
}
