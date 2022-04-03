#include <iostream>
#include <vector>
#include <map>
#include <string>
using namespace std;

struct MyTreeNode {
     char letter;
     map<char, MyTreeNode*> leaves;
     MyTreeNode() : letter('0'), leaves() {}
     MyTreeNode(char c) : letter(c), leaves() {}
};

class Solution {
public:
    string longestCommonPrefix(vector<string>& strs) {
        MyTreeNode root;
        // Fill-up tree
        for (string word : strs) {
            if (word.empty())
                return "";
            addWord(word, &root);
        }
        
       
        // Find longest prefix
        string prefix = "";
        MyTreeNode *iter = &root;
        while (iter->leaves.size() == 1) {
            auto el = iter->leaves.begin();
            if (el->first != '0')
                prefix.append(1, el->first);
            iter = el->second;
        }

        return prefix;
    }

    void addWord(string &s, MyTreeNode *tree) {
        MyTreeNode *iter = tree;

        for (char c : s) {
            if (iter->leaves.size() > 1) {
                break;
            }
            if (iter->leaves.count(c) == 0) {
                iter->leaves[c] = new MyTreeNode(c);
            }
            iter = iter->leaves[c];
        }
        iter->leaves['0'] = new MyTreeNode('0');
    }

    void printTree(MyTreeNode *tree, int depth) {
    for (int i=0; i<depth; i++)
        cout << "----";
    cout << tree->letter << endl;
    for (auto &el : tree->leaves)
        printTree(el.second, depth+1);
}
};


int main() {
    Solution solution;
    //vector<string> testSet = {"flower","flow","flight"};
    vector<string> testSet = {"", "a"};

    cout << "Longest common prefix among the following list:" << endl;
    cout << "[";
    for (string s : testSet)
        cout << s << ", ";
    cout << "]" << endl;

    cout << "Longest prefix is '" << solution.longestCommonPrefix(testSet) << "'" << endl;

}