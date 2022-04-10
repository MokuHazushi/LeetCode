#include <vector>
#include <iostream>
using namespace std;

class Solution {
public:
    void sortColors(vector<int>& nums) {
        int start, end, iter;
        
        start = iter = 0;
        end = nums.size()-1;
        
        while (iter <= end) {
            switch (nums[iter]) {
                case 0:
                    nums[iter] = nums[start];
                    nums[start] = 0;
                    start++;
                    iter++;
                break;
                case 2:
                    nums[iter] = nums[end];
                    nums[end] = 2;
                    end--;
                    break;
                default:
                    iter++;
            }
        }
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
    vector<vector<int>> testSet;
    testSet.push_back({0, 1, 2});
    testSet.push_back({2, 1, 0});
    testSet.push_back({1, 0, 1});
    testSet.push_back({1, 1, 1});
    testSet.push_back({2, 1, 1});
    testSet.push_back({2, 2, 1, 1, 0, 0});

    cout << "Sort colors:" << endl;
    for (vector<int> set : testSet) {
        cout << "Input: ";
        printVector(set);
        cout << endl;
        solution.sortColors(set);
        cout << "Output: ";
        printVector(set);
        cout << endl << endl;
    }
}