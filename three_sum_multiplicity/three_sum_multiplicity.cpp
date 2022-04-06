#include <iostream>
#include <my_utils.h>
#include <map>
#include <vector>
#include <cmath>

class Solution {
public:
    int threeSumMulti(vector<int>& arr, int target) {
		map<int, int> counter1, counter2;
		int result, modulo;

		modulo = pow(10, 9) + 7;
		result = 0;
		for (int val : arr) {
			if (counter2.count(target-val) != 0)
				result = (result + counter2[target-val] ) % modulo;
			for (auto it=counter1.begin(); it!=counter1.end(); ++it) {
				counter2[it->first+val] += it->second; // second member of triplet (e.g. number of pairs)
			}
			counter1[val]++; // first member of triplet
		}

		return result;
	}

};


int main() {
	Solution solution;
	vector<pair<int, vector<int>>> testSet;

	testSet.push_back(pair<int, vector<int>>(1, vector<int>({1,0,0})));
	testSet.push_back(pair<int, vector<int>>(1, vector<int>({1,0,0,0,0})));
	testSet.push_back(pair<int, vector<int>>(3, vector<int>({1,1,1})));
	testSet.push_back(pair<int, vector<int>>(3, vector<int>({1,1,1,1,1})));
	testSet.push_back(pair<int, vector<int>>(8, vector<int>({1,1,2,2,3,3,4,4,5,5})));
	testSet.push_back(pair<int, vector<int>>(8, vector<int>({1,1,2,2,3,3,4,4,5,5,5})));
	testSet.push_back(pair<int, vector<int>>(5, vector<int>({1,1,2,2,2})));
	testSet.push_back(pair<int, vector<int>>(5, vector<int>({1,1,2,2,2,2})));

	cout << "Counting number of triplets (i,j,k) i<j<k such that t[i]+t[j]+t[k] = target:" << endl;
	for (auto test : testSet) {
		cout << NumberVectorToString(test.second) << " target=" << test.first << " -> " << solution.threeSumMulti(test.second, test.first) << endl;
	}
}
