#include <iostream>
#include <utility>
#include <vector>
#include <array>
#include <algorithm>

#include <my_utils.h>
using namespace std;

struct My_MinMax {
    int minVal, minIndex, maxVal, maxIndex;
    
    My_MinMax(int a, int b, int c, int d) : minVal(a), minIndex(b), maxVal(c), maxIndex(d) {}
};

class Solution {
public:
    void nextPermutation(vector<int>& nums) {          
        if (nums.size() == 1)
            return;
        
        array<bool, 101> existsInMinMaxRange;
        bool swapped = false;
        int iter = nums.size()-2;
        My_MinMax minmax(nums[iter+1], iter+1, nums[iter+1], iter+1);
        existsInMinMaxRange.fill(false);
        existsInMinMaxRange[nums[iter+1]] = true;
        
        while (iter >= 0) {

            if (existsInMinMaxRange[nums[iter]] && minmax.maxVal == nums[iter]) {
                iter--;
                continue;
            }

            if (nums[iter] < minmax.minVal) {
                swapIndexes(nums, iter, minmax.minIndex);
                swapped = true;
                break;
            }
            else if (nums[iter] > minmax.maxVal) {

                if (iter == 0) {
                    swapIndexes(nums, iter, minmax.minIndex);
                    swapped = true;
                    break;
                }

                minmax.maxIndex = iter;
                minmax.maxVal = nums[iter];
                existsInMinMaxRange[nums[iter]] = true;
            }
            else {
                int indexToSwap = findClosestValue(nums, nums[iter], iter+1, nums.size()-1);
                swapIndexes(nums, iter, indexToSwap);
                swapped = true;
                break;
            }
            iter--;
        }
        
        if (!swapped) {
            int indexToSwap = findClosestValue(nums, nums[0], 0, nums.size()-1);
            swapIndexes(nums, 0, indexToSwap);
            iter = 0;
        }
        
        if (nums.size() - iter > 1)
            sort(nums.begin()+iter+1, nums.end());
    }
    
    void swapIndexes(vector<int> &nums, int a, int b) {
        int tmp = nums[a];
        nums[a] = nums[b];
        nums[b] = tmp;
    }
    
    /* Search index k (i <= k <= j) such that nums[k]-n  is minimal */
    int findClosestValue(vector<int> &nums, int n, int i, int j) {
        int diff = 101;
        int result = 0;
        for (int k=i; k<=j; k++) {
            int tmp = nums[k] - n ;
            if (tmp > 0 && tmp < diff) {
                diff = abs(nums[k]-n);
                result = k;
            }
        }
        return result;
    }
};
int main() {
    Solution solution;
    vector<vector<int>> testSet;

    testSet.push_back(vector<int>{1,2,3});
    testSet.push_back(vector<int>{1});
    testSet.push_back(vector<int>{1,5,7,6});
    testSet.push_back(vector<int>{1,2,4,3});
    testSet.push_back(vector<int>{1,9,7,6});
    testSet.push_back(vector<int>{2,3,1});
    testSet.push_back(vector<int>{5,7,6,2,1});
    testSet.push_back(vector<int>{1,3,2});
    testSet.push_back(vector<int>{5,1,1});
    testSet.push_back(vector<int>{3,6,6,2});
    testSet.push_back(vector<int>{1,5,1});
    testSet.push_back(vector<int>{1,5,4,3,1});
    testSet.push_back(vector<int>{2,2,7,5,4,3,2,2,1});
    testSet.push_back(vector<int>{2,1,2,2,2,2,2,1});

    cout << "Next permutation (in lexicographically order)" << endl;
    for (vector<int> test : testSet) {
        cout << NumberVectorToString(test) << " -> ";
        solution.nextPermutation(test);
        cout << NumberVectorToString(test) << endl;
    }
}