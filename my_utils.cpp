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