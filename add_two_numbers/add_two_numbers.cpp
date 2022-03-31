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
        ListNode *result, *iter;
        int remainder;
        
        remainder = 0;
        result = new ListNode();
        iter = result;
        while (true) {
            int v1, v2, sum;
            
            if (l1 == nullptr && l2 == nullptr) {
                if (remainder != 0)
                    iter->next = new ListNode(remainder);
                return result;
            }
        
            v1 = v2 = 0;
            if (l1 != nullptr)
                v1 = l1->val;
            if (l2 != nullptr)
                v2 = l2->val;
            
            sum = v1+v2+remainder;
            if (sum > 9) {
                remainder = 1;
                iter->val = sum%10;
            }
            else {
                remainder = 0;
                iter->val = sum;
            }
            
            if (l1 != nullptr)
                l1 = l1->next;
            if (l2 != nullptr)
                l2 = l2->next;
            
            if (!(l1 == nullptr && l2 == nullptr)) {
                iter->next = new ListNode();
                iter = iter->next;
            }
        }
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

    void print(ListNode* l) {
        ListNode* tmp = l;
        while (tmp != nullptr) {
            cout << tmp->val << endl;
            tmp = tmp->next;
        }
    }
};

int main() {
    Solution solution;
    vector<int> num1 {2, 4, 3};
    vector<int> num2 {5, 6, 4};
    ListNode *l1 = solution.convert(num1);
    ListNode *l2 = solution.convert(num2);

    ListNode *result = solution.addTwoNumbers(l1, l2);

    cout << "Result" << endl;
    while (result != nullptr) {
        cout << result->val << endl;
        result = result->next;
    }
}