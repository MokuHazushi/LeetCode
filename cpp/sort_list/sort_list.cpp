#include <iostream>
#include <my_utils.h>
#include <tgmath.h>
#include <chrono>
#include <algorithm>

class Solution {
public:
    ListNode* sortList(ListNode* head) {
		if (head == nullptr)
			return nullptr;

		vector<ListNode*> tab(16, nullptr);
		ListNode *result, *iter;
		int i;

		result = head;
		while (result != nullptr) {
			iter = result->next;
			result->next = nullptr;
			for (i=0; i<16 && tab[i] != nullptr; i++) {
				result = merge(tab[i], result);
				tab[i] = nullptr;
			}
			if (i == 16)
				i--;
			
			tab[i] = result;
			result = iter;
		}

		result = nullptr;
		for (i=0; i<16; i++) {
			if (tab[i] == nullptr)
				continue;
			if (result == nullptr)
				result = tab[i];
			else
				result = merge(result, tab[i]);
		}
		return result;
	}

	ListNode* merge(ListNode* left, ListNode* right) {
		ListNode *result, *resultIter;

		result = resultIter = nullptr;
		while (left != nullptr && right != nullptr) {
			ListNode* toAppend;
			if (left->val <= right->val) {
				toAppend = left;
				left = left->next;
			}
			else {
				toAppend = right;
				right = right->next;
			}
			
			if (result == nullptr) {
				result = toAppend;
				resultIter = result;
			}
			else {
				resultIter -> next = toAppend;
				resultIter = resultIter->next;
			}
			resultIter->next = nullptr;
		}

		while (left != nullptr) {
			resultIter->next = left;
			resultIter = resultIter->next;
			left = left->next;
		}
		while (right != nullptr) {
			resultIter->next = right;
			resultIter = resultIter->next;
			right = right->next;
		}

		resultIter->next = nullptr;

		return result;
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
		list = solution.sortList(list);
		vector<int> sortedList = LinkedListToVector(list);
		cout << "Sorting " << NumberVectorToString(test) << endl;
		cout << "\t-> " << NumberVectorToString(sortedList) << endl;
	}

	cout << "Benchmark testing" << endl;
	vector<vector<int>> benchmarkSet;
	vector<int> largeAlmostSortedVector;
	ListNode* largeList;

	for (int i=2; i<=5*pow(10, 4); i++) 
		largeAlmostSortedVector.push_back(i);
	largeAlmostSortedVector.push_back(1);

	largeList = VectorToLinkedList(largeAlmostSortedVector);
	cout << "Sorting list of size=" << largeAlmostSortedVector.size() << endl;
	auto chronoStart = chrono::high_resolution_clock::now();
	solution.sortList(largeList);
	auto chronoStop = chrono::high_resolution_clock::now();
	auto duration = chrono::duration_cast<chrono::milliseconds>(chronoStop - chronoStart);
	cout << "\tTook " << duration.count() << "ms" << endl;

	cout << "CPP std library sort took: ";
	chronoStart = chrono::high_resolution_clock::now();
	sort(largeAlmostSortedVector.begin(), largeAlmostSortedVector.end());
	chronoStop = chrono::high_resolution_clock::now();
	duration = chrono::duration_cast<chrono::milliseconds>(chronoStop - chronoStart);
	cout << duration.count() << "ms" << endl;
}
