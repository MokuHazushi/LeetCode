#include <iostream>
#include <my_utils.h>

class Solution {
public:
    ListNode* sortList(ListNode* head) {
		vector<int> list;
		ListNode *pivot, *init;

		if (head == nullptr)
			return nullptr;

		if (head->next == nullptr)
			return head;
		
		// Initialize pivot to last element
		pivot = init = head;
		while (pivot->next != nullptr)
			pivot = pivot->next;

		quickSort(init, pivot);
		return head;
    }

	void quickSort(ListNode *low, ListNode* high) {

		if (low != high) {
			ListNode *pivot = partition(low, high);

			quickSort(low, pivot);
			quickSort(pivot->next, high);
		}

	}

	ListNode* partition(ListNode *low, ListNode *high) {
		ListNode *i = nullptr;
		ListNode *iter = low;

		while (iter != high) {
			if (iter->val < high->val) {
				if (i == nullptr)
					i = low;
				else
					i = i->next; 
				swap(i, iter);
			}
			iter = iter->next;
		}

		if (i == nullptr) {
			i = low;
			swap(i, high);
		}
		else
			swap(i->next, high);

		return i;
	}

	void swap(ListNode *node1, ListNode *node2) {
		int tmp = node1->val;
		node1->val = node2->val;
		node2->val = tmp;
	}
};

int main() {
	Solution solution;
	vector<vector<int>> testSet;

	testSet.push_back(vector<int>({2,4,3,1}));
	testSet.push_back(vector<int>({1,2,3,4}));
	testSet.push_back(vector<int>({4,3,2,1}));
	testSet.push_back(vector<int>({2,1,1,1}));
	testSet.push_back(vector<int>({1}));
	testSet.push_back(vector<int>({}));

	cout << "Sorting linked list:" << endl;
	for (vector<int> test : testSet) {
		ListNode* list = VectorToLinkedList(test);
		solution.sortList(list);
		vector<int> sortedList = LinkedListToVector(list);
		cout << "Sorting " << NumberVectorToString(test) << endl;
		cout << "\t-> " << NumberVectorToString(sortedList) << endl;
	}
}
