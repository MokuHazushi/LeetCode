#include <iostream>
#include <my_utils.h>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    ListNode* mergeKLists(vector<ListNode*>& lists) {
		vector<ListNode*> heap;


		for (ListNode *l : lists) {
			if (l != nullptr)
				heap.push_back(l);
		}

		if (heap.empty())
			return nullptr;

		ListNode *result, *iter;

		result = nullptr;
		make_heap(heap.begin(), heap.end(), list_comparator);

		while (!heap.empty()) {
			ListNode *next = heap.front();

			if (result == nullptr) {
				result = next;
				iter = result;
			}
			else {
				iter->next = next;
				iter = iter->next;
			}
			
			pop_heap(heap.begin(), heap.end(), list_comparator);
			heap.pop_back();

			if (next->next != nullptr) {
				heap.push_back(next->next);
				push_heap(heap.begin(), heap.end(), list_comparator);
			}
		}

		iter->next = nullptr;
		return result;
    }

	static bool list_comparator(ListNode *l1, ListNode *l2) {
		return l1->val > l2->val;
	}
};

int main() {
	Solution solution;
	vector<vector<vector<int>>> testSet;

	testSet.push_back(vector({
		vector({1,2,3}), 
		vector({1,2,3})
	}));
	testSet.push_back(vector({
		vector({1,2,3}), 
		vector({1,2,3}),
		vector({2,3})
	}));

	cout << "Merging multiple linked list" << endl;
	for (vector<vector<int>> test : testSet) {
		vector<ListNode*> lists;
		vector<int> result;
		cout << "Merging:" << endl;
		for (vector<int> l : test) {
			cout << "\t" << NumberVectorToString(l) << endl;
			lists.push_back(VectorToLinkedList(l));
		}
		result = LinkedListToVector(solution.mergeKLists(lists));
		cout << "\t --> " << NumberVectorToString(result) << endl;
	}
}
