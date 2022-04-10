#include <iostream>
#include <my_utils.h>
#include <map>
#include <algorithm>
#include <vector>
#include <iterator>

class Solution {
public:
    vector<int> topKFrequent(vector<int>& nums, int k) {
		map<int, int> frequency;
		vector<pair<int,int>> pairs;
		vector<int> result;

		for (int n : nums)
			frequency[n]++;
		
		copy(frequency.begin(), frequency.end(), back_inserter<vector<pair<int, int>>>(pairs));
		sort(pairs.begin(), pairs.end(), 
		[](auto p1, auto p2){ return p1.second > p2.second;});

		for (int i=0; i<k; i++) 
			result.push_back(pairs[i].first);
		
		return result;
    }
};

int main() {
	Solution solution;
	vector<pair<int, vector<int>>> testSet;

	testSet.push_back(pair(2, vector<int>({1,1,1,2,2,3})));
	testSet.push_back(pair(1, vector<int>({1,1,1,2,2,3})));

	cout << "k most frequent elements in list:" << endl;
	for (pair<int, vector<int>> test : testSet) {
		vector<int> result = solution.topKFrequent(test.second, test.first);
		cout << "k=" << test.first << " tab=" << NumberVectorToString(test.second) << endl;
		cout << "\tMost frequent elements are " << NumberVectorToString(result) << endl;
	}
}
