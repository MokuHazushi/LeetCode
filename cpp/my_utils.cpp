#include <vector>
#include <string>
#include <iostream>
#include <queue>

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

/* Return string representing the tree in breadth traversal order */
string NumberBTToString(TreeNode* node) {
    queue<TreeNode*> breadthTraversal;
    string s("[");
    
    breadthTraversal.push(node);
    while (!breadthTraversal.empty()) {
        TreeNode *head = breadthTraversal.front();
        breadthTraversal.pop();

        if (head == nullptr) {
            s.append("null, ");
            continue;
        }
        else {
            s.append(to_string(head->val));
            s.append(", ");
        }
        breadthTraversal.push(head->left);
        breadthTraversal.push(head->right);
    }

    s.erase(s.end()-2, s.end());
    s.append("]");
    return s;
}