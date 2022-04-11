#include <iostream>
#include <my_utils.h>
#include <vector>
#include <map>
#include <utility>
#include <algorithm>
#include <chrono>

class Solution {
public:
    vector<vector<int>> fourSum(vector<int>& nums, int target) {
        vector<vector<int>> result;
        map<int, int> elements;
		vector<int> uniqEls;

		for (int n : nums)
			elements[n] = min(4, elements[n]+1);

		for (auto it=elements.begin(); it!=elements.end(); it++)
			uniqEls.push_back(it->first);

		for (int a=0; a<uniqEls.size(); a++) {
			int b, valA;
			bool decrementedA = false;
			valA = uniqEls[a];
			if (elements[valA] > 1) {
				b = a;
				elements[valA]--;
				decrementedA = true;
			}
			else
				b = a+1;
			
			while (b < uniqEls.size()) {
				int c, valB;
				bool decrementedB = false;
				valB = uniqEls[b];
				if (elements[valB] > 1) {
					c = b;
					elements[valB]--;
					decrementedB = true;
				}
				else
					c = b+1;
				
				while (c < uniqEls.size()) {
					int d, valC;
					bool decrementedC = false;
					valC = uniqEls[c];
					if (elements[valC] > 1) {
						d = c;
						elements[valC]--;
						decrementedC = true;
					}
					else
						d = c+1;
					
					while (d < uniqEls.size()) {
						int valD = uniqEls[d];
                        long tmp = valA/10 + valB/10 + valC/10 + valD/10;
                        if (abs(10*tmp) >= 2147483647) {
                            d++;
                            continue;
                        }
						if (valA + valB + valC + valD == target)
							result.push_back(vector<int>({valA, valB, valC, valD}));
						d++;
					}

					if (decrementedC)
						elements[valC]++;
					c++;
				}

				if (decrementedB)
					elements[valB]++;
				b++;

			}

			if (decrementedA)
				elements[valA]++;
		}

		return result;
    }
};


int main() {
	Solution solution;
	vector<pair<int, vector<int>>> testSet;

	//testSet.push_back(pair<int, vector<int>>(8, vector<int>{2,2,2,2,2}));
	testSet.push_back(pair<int, vector<int>>(0, vector<int>{1,0,-1,0,-2,2}));
	testSet.push_back(pair<int, vector<int>>(0, vector<int>{-2,-1,-1,1,1,2,2}));

	cout << "Searching unique 4 sums:" << endl;
	for (auto test : testSet) {
		vector<vector<int>> result;
		cout << "Search sum=" << test.first << " in " << NumberVectorToString(test.second) << endl;
		cout << "\t -> ";
		result = solution.fourSum(test.second, test.first);
		for (vector<int> s : result)
			cout << NumberVectorToString(s) << " ; ";
		cout << endl;
	}

	vector<int> largeVec;
	int k = 0;
	for (int i=0; i<200; i++)
		largeVec.push_back(i);
	
	cout << "Try with a vector of size 200, all unique elements" << endl;
	auto chronoStart = chrono::high_resolution_clock::now();
	solution.fourSum(largeVec, k);
	auto chronoStop = chrono::high_resolution_clock::now();
	auto duration = chrono::duration_cast<chrono::milliseconds>(chronoStop - chronoStart);
	cout << "Time taken is " << duration.count() << "ms" << endl;
}
