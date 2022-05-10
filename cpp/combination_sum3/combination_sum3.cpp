// https://leetcode.com/problems/combination-sum-iii/

#include <iostream>
#include <my_utils.h>
#include <vector>

class Solution {
public:
    vector<vector<int>> combinationSum3(int k, int n) {
		vector<vector<int>> solution;

		for (int i=1; i<=9; i++)
			backtrackSearch(k, n, solution, i, vector<int>({i}));

		return solution;
    }

	void backtrackSearch(int k, int n, vector<vector<int>> &solution, int accSum, vector<int> nums) {
		if (nums.size() == k) {
			if (accSum == n) {
				solution.push_back(nums);
				return;
			}
			else
				return;
		}

		for (int i=nums[nums.size()-1]+1; i<=9; i++) {
			vector<int> newNums(nums);
			newNums.push_back(i);
			backtrackSearch(k, n, solution, accSum+i, newNums);
		}
	}
};

struct Test {
	int k, n;
	Test(int k, int n) : k(k), n(n) {}
};

int main() {
	Solution solution;
	vector<Test> testSet;

	testSet.push_back(Test(5, 23));

	for (Test test : testSet) {
		cout << "k=" << test.k << " n=" << test.n << endl;
		for (vector<int> combination : solution.combinationSum3(test.k, test.n))
			cout << NumberVectorToString(combination) << ", ";
		cout << endl;
	}
}
