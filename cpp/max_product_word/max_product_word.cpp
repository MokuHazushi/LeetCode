// https://leetcode.com/problems/maximum-product-of-word-lengths/

#include <iostream>
#include <my_utils.h>
#include <vector>
#include <string>
#include <algorithm>
#include <map>

class Solution {
public:
    int maxProduct(vector<string>& words) {
		map<int, int> entries;
		int maxProduct = 0;

		for (string s : words) {
			int letterMask = 0;
			
			for (char c : s)
				letterMask |= 1 << (c-'a');

			if (entries.count(letterMask) == 0)
				entries[letterMask] = s.size();
			else
				entries[letterMask] = max(entries[letterMask], (int)s.size());
		}

		for (auto i=entries.begin(); i!=entries.end(); i++) {
			for (auto j=i; j!=entries.end(); j++) {

				if ((i->first & j->first) == 0)
					maxProduct = max(maxProduct, i->second*j->second);
			}
		}

		return maxProduct;
    }
};

int main() {
	cout << "Find max product in list of string" << endl;
	Solution solution;
	vector<vector<string>> testSet;

	testSet.push_back(vector<string>({"ac", "bd"}));
	testSet.push_back(vector<string>({"abcw", "baz", "foo", "bar", "xtfn", "abcdef"}));
	testSet.push_back(vector<string>({"a", "aa", "aaa"}));

	for (vector<string> test : testSet) {
		cout << "words=" << StringVectorToString(test) << endl;
		cout << "\t-> " << solution.maxProduct(test) << endl;
	}
}
