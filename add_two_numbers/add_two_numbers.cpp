#include <vector>
#include <iostream>
using namespace std;

struct ListNode {
     int val;
     ListNode *next;
     ListNode() : val(0), next(nullptr) {}
     ListNode(int x) : val(x), next(nullptr) {}
     ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        bool hasRemainder = false;
        vector<int> result, remainder;
        
        do {
            hasRemainder = false;
            do {
                int val1, val2;
                val1 = val2 = 0;
                if (l1 != nullptr)
                    val1 = l1->val;
                if (l2 != nullptr)
                    val2 = l2->val;
                int sum = val1 + val2;
                if (sum > 9) {
                    result.push_back(sum%10);
                    remainder.push_back(1);
                    hasRemainder = true;
                }
                else {
                    result.push_back(sum);
                    remainder.push_back(0);
                }
                
                if (l1 != nullptr)
                    l1 = l1->next;
                if (l2 != nullptr)
                    l2 = l2->next;
            }
            while (l1 != nullptr && l2 != nullptr);
            
            if (hasRemainder) {
                l1 = convert(result);
                l2 = convertRemainder(remainder);
            }
        } while (hasRemainder);
            
            return convert(result);
    }
    
    ListNode* convert(vector<int> v) {
        ListNode *iter, *list;
        
        list = new ListNode();
        iter = list;
        for (size_t i=0; i<v.size()-1; i++) {
            iter->val = v[i];
            iter->next = new ListNode();
            iter = iter->next;
        }
        iter->val = v[v.size()-1];
        return list;
    }
    
    ListNode* convertRemainder(vector<int> v) {
        ListNode *iter, *list;
        
        list = new ListNode();
        iter = list;
        for (size_t i=0; i<v.size(); i++) {
            iter->val = v[i];
            iter->next = new ListNode();
            iter = iter->next;
        }
        return list;
    }
};

int main() {
    Solution solution;
    vector<int> num1 {5, 5};
    vector<int> num2 {7, 7};
    ListNode *l1 = solution.convert(num1);
    ListNode *l2 = solution.convert(num2);

    ListNode *result = solution.addTwoNumbers(l1, l2);

    while (result != nullptr) {
        cout << result->val << endl;
        result = result->next;
    }
}