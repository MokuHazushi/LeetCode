#include <iostream>
#include <utility>
#include <vector>
using namespace std;

class Solution {
public:
    int searchInsert(vector<int>& nums, int target) {
        int start, end, middle;
        
        start = middle = 0;
        end = nums.size();
        
        while ((end - start) > 1) {
            middle = start + ((end - start)/2);
            
            if (target == nums[middle])
                return middle;
            if (target < nums[middle])
                end = middle;
            if (target > nums[middle])
                start = middle;
        }        
        
        if (target <= nums[start])
            return start;
        return end++;
    }
};

void printVector(vector<int> v) {
    cout << "[";
    for (int i=0; i<v.size()-1; i++)
        cout << v[i] << ", ";
    cout << v[v.size()-1] << "]";
}

int main() {
    Solution solution;
    vector<pair<int, vector<int>>> testSet;

    testSet.push_back(pair(1, vector<int>{1, 3, 5, 6}));
    testSet.push_back(pair(2, vector<int>{1, 3, 5, 6}));
    testSet.push_back(pair(3, vector<int>{1, 3, 5, 6}));
    testSet.push_back(pair(4, vector<int>{1, 3, 5, 6}));
    testSet.push_back(pair(5, vector<int>{1, 3, 5, 6}));
    testSet.push_back(pair(6, vector<int>{1, 3, 5, 6}));
    testSet.push_back(pair(7, vector<int>{1, 3, 5, 6}));
    testSet.push_back(pair(0, vector<int>{1}));
    testSet.push_back(pair(1, vector<int>{1}));
    testSet.push_back(pair(2, vector<int>{1}));

    cout << "Search insert" << endl;
    for (auto test : testSet) {
        cout << "Insert '" << test.first << "' in vector ";
        printVector(test.second);
        cout << endl;
        cout << "\tInsert position is index=" << solution.searchInsert(test.second, test.first) << endl;
    }
}