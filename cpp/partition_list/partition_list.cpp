// https://leetcode.com/problems/partition-list/

#include <iostream>
#include <vector>
#include <my_utils.h>

class Solution {
public:
	ListNode* partition(ListNode* head, int x) {
		ListNode *insertionPoint, *iter, *prev;
        
        insertionPoint = nullptr;
		iter = head;

		while (iter != nullptr) {
			if (iter->val >= x) {
				break;
			}
            insertionPoint = iter;
			iter = iter->next;
		}
        
		if (iter == nullptr)
			return head;

        prev = iter;
        iter = iter->next;
		while (iter != nullptr) {
			ListNode *next = iter->next;
			if (iter->val < x) {
                if (insertionPoint == nullptr) {
				    insertionPoint = replace(insertionPoint, iter, prev, head);
                    head = insertionPoint;
                }
                else
                    insertionPoint = replace(insertionPoint, iter, prev, head);
			}
            else
                prev = iter;            
			iter = next;
		}
		return head;        
    }

	ListNode* replace(ListNode *insertionPoint, ListNode *iter, ListNode* prev, ListNode *head) {
        prev->next = iter->next;
		if (insertionPoint == nullptr) {
			iter->next = head;
			return iter;
		}

		iter->next = insertionPoint->next;
		insertionPoint->next = iter;
		return iter;
	}
};


int main() {
	Solution solution;
}
