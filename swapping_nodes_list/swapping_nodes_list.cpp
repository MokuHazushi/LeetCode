#include <iostream>
#include <vector>
#include <utility>
#include <my_utils.h>

class Solution {
public:
        ListNode* swapNodes(ListNode* head, int k) {
        
        if (head->next == nullptr)
            return head;
        
        ListNode *iter, *tmp;
        int size, i, indexFromTail;
        
        size = 0;
        iter = head;
        while (iter != nullptr) {
            size++;
            iter = iter->next;
        }
        
        iter = head;
        indexFromTail = size-k+1;
        i = 0;
        
        if (indexFromTail <= size/2) {
            k = indexFromTail;
            indexFromTail = size-k+1;
        }
        
        while (iter != nullptr) {
            i++;
            
            if (i == k)
                tmp = iter;
            
            if (i == indexFromTail) {
                int tmpVal = iter->val;
                iter->val = tmp->val;
                tmp->val = tmpVal;
                return head;
            }
            
            iter = iter->next;
        }
        return head;  
    }
};

int main() {
	Solution solution;
	vector<pair<int, vector<int>>> testSet;

	testSet.push_back(pair(1, vector<int>{1,2,3}));
	testSet.push_back(pair(2, vector<int>{1,2,3}));
	testSet.push_back(pair(3, vector<int>{1,2,3}));
	testSet.push_back(pair(1, vector<int>{1,2,3,4}));
	testSet.push_back(pair(2, vector<int>{1,2,3,4}));
	testSet.push_back(pair(3, vector<int>{1,2,3,4}));
	testSet.push_back(pair(4, vector<int>{1,2,3,4}));

	cout << "Swap at indexes (k, size-k+1) in linked list:\n";
	for (auto p : testSet) {
		ListNode *list;
		vector<int> v;
		cout << "List " << NumberVectorToString(p.second) << " k=" << p.first;
		list = VectorToLinkedList(p.second);
		list = solution.swapNodes(list, p.first);
		v = LinkedListToVector(list);
		cout << " -> " << NumberVectorToString(v) << endl;
	}

}
