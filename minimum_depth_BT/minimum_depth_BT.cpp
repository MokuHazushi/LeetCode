#include <iostream>
#include <vector>
using namespace std;

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class Solution {
public:
    int minDepth(TreeNode* root) {
        
        if (root == nullptr)
            return 0;
        
        int depth = 1;
        vector<TreeNode*> toExplore;
        vector<TreeNode*> tmp;
        
        toExplore.push_back(root);
        while (!toExplore.empty()) {
            for (TreeNode *node : toExplore) {
                if (isLeaf(node))
                    return depth;
                if (node->left != nullptr)
                    tmp.push_back(node->left);
                if (node->right != nullptr)
                    tmp.push_back(node->right);
            }
            swap(toExplore, tmp);
            tmp.clear();
            depth++;
        }
        return depth;
    }
    
    bool isLeaf(TreeNode *t) {
        return t->left == nullptr && t->right == nullptr;
    }
};

int main() {
    Solution solution;

    // TODO
}