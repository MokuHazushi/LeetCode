#include <vector>
#include <string>
#include <iostream>

#include <my_utils.h>

using namespace std;

ListNode* VectorToLinkedList(vector<int> &v) {
    ListNode *head, *iter;

    if (v.empty())
        return nullptr;
    
    head = new ListNode(v[0]);
    iter = head;
    for (int i=1; i<v.size(); i++) {
        iter-> next = new ListNode(v[i]);
        iter = iter->next;
    }

    return head;
}

vector<int> LinkedListToVector(ListNode *l) {
    vector<int> result;
    ListNode *iter;
    
    iter = l;
    while (iter != nullptr) {
        result.push_back(iter->val);
        iter = iter->next;
    }

    return result;
}

string booleanToString(bool b) {
    if (b)
        return string("TRUE");
    return string("FALSE");
}

void NumberBTToString_rec(TreeNode* node, string &acc) {
    if (node == nullptr)
        return;
    acc.append(to_string(node->val));
    acc.append(", ");
    NumberBTToString_rec(node->left, acc);
    NumberBTToString_rec(node->right, acc);
}

/*
 * Create the string in Breadth first search order
 */
string NumberBTToString(TreeNode* root) {
    string s("[");
    TreeNode *iter = root;
    NumberBTToString_rec(iter, s);
    s.erase(s.end()-2);
    s.append("]");
    return s;
}