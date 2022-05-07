// https://leetcode.com/problems/132-pattern/

#include <iostream>
#include <my_utils.h>
#include <vector>
#include <stack>
#include <algorithm>

class Solution {
public:
    bool find132pattern(vector<int>& nums) {
		/*
		* best_i[j] = x is a value in nums such that
		* x < nums[j] and x is minimal in nums[0..j]
		*/
		vector<int> best_i(nums.size(), -1);
		stack<int> stack_k;
		best_i[0] = nums[0]; // nums is at least of size 1

		for (int i=1; i<nums.size(); i++)
			best_i[i] = min(best_i[i-1], nums[i]);

		
		for (int j=nums.size()-1; j>=0; j--) {
			if (nums[j] <= best_i[j])
				continue;
			
			while (!stack_k.empty() && stack_k.top() <= best_i[j])
				stack_k.pop();
			
			if (!stack_k.empty() && stack_k.top() < nums[j])
				return true;
			
			stack_k.push(nums[j]);
		}

		return false;

    }
};

int main() {
	Solution solution;
	vector<vector<int>> test_set;

	test_set.push_back(vector<int>({1,2,3,4}));
	test_set.push_back(vector<int>({3,1,4,2}));
	test_set.push_back(vector<int>({-1,3,2,0}));
	test_set.push_back(vector<int>({-1,4,-2,3,1,1,1,1,2}));
	test_set.push_back(vector<int>({-1,4,-2,-1,-2}));
	test_set.push_back(vector<int>({-1,4,-2,-2,2}));
	test_set.push_back(vector<int>({-1,4,-3,5,-1}));
	test_set.push_back(vector<int>({-2,1,1}));
	test_set.push_back(vector<int>({1,4,0,-1,-2,-3,-1,-2}));
	test_set.push_back(vector<int>({1,3,2,4,5,6,7,8,9,10}));
	test_set.push_back(vector<int>({3,5,0,3,4}));

	for (vector<int> test : test_set) {
		cout << "nums=" << NumberVectorToString(test);
		cout << " -> " << booleanToString(solution.find132pattern(test)) << endl;
	}
}
