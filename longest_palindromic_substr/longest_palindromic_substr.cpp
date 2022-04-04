#include <iostream>
#include <vector>
#include <string>
#include <array>
#include <my_utils.h>

class Solution {
public:
    string longestPalindrome(string s) {

		if (s.size() == 1)
			return s;

        bool **dynamicTab;
		int i, j, k;

		dynamicTab = allocTab(s.size());

		// Fill dynamic programming table
		for (i=0; i<s.size(); i++)
			dynamicTab[i][i] = true;

		i = 0;
		j = 1;
		k = 1;
		while (k < s.size()) {		
			if (j-i == 1)
				dynamicTab[i][j] = s[i] == s[j];
			else
				dynamicTab[i][j] = dynamicTab[i+1][j-1] && (s[i] == s[j]);

			i++;
			j++;

			if (j == s.size()) {
				k++;
				i = 0;
				j = k;
			}
		}

		// Find best solution in table
		i = 0;
		j = s.size()-1;
		while (k > 0) {
			if (dynamicTab[i][j]) {
				free(dynamicTab);
				return s.substr(i, j-i+1);
			}
			
			i++;
			j++;
			if (j == s.size()) {
				k--;
				i = 0;
				j = k;
			}
		}

		free(dynamicTab);
		return string(1, s[0]);

    }
    
    bool isPalindrome(string &s, int i, int j) {
        while (i < j) {
            if (s[i] != s[j])
                return false;
            i++;
            j--;
        }
        return true;
    }

	bool** allocTab(int size) {
		bool** tab;
		tab = (bool**)malloc(sizeof(bool*) * size);
		for (int i = 0; i<size; i++)
			tab[i] = (bool*)malloc(sizeof(bool));

		return tab;
	}

	void freeTab(bool **tab, int size) {
		for (int i=0; i<size; i++)
			free(tab[i]);
		free(tab);
	}
};

int main() {
	Solution solution;
	vector<string> testSet;

	testSet.push_back("babad");
	testSet.push_back("1234");
	testSet.push_back("cbbd");
	testSet.push_back("aa12321");

	cout << "Search longest palindromic substring:" << endl;
	for (string s : testSet) {
		cout << "'" << s << "' -> " << solution.longestPalindrome(s) << endl;
	}
}
