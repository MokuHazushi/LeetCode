// https://leetcode.com/problems/palindromic-substrings/

#include <iostream>
#include <my_utils.h>
#include <string>
#include <vector>

class Solution {
public:
    int countSubstrings(string s) {
		int total = 0;
		for (int i=0; i<s.size(); i++)
			total += 1 + countEvenPalindroms(s, i) + countOddPalindroms(s, i);
		
        return total;
    }

	int countEvenPalindroms(string &s, int pos) {
		int total = 0;
		int left = pos;
		int right = pos+1;

		while (left >= 0 && right <= s.size()-1 && s[left] == s[right]) {
			total++;
			left--;
			right++;
		}
		return total;
	}

	int countOddPalindroms(string &s, int pos) {
		int total = 0;
		int left = pos-1;
		int right = pos+1;

		while (left >= 0 && right <= s.size()-1 && s[left] == s[right]) {
			total++;
			left--;
			right++;
		}
		return total;
	}
};

int main() {
	Solution solution;
	vector<string> testSet;

	testSet.push_back(string("abc"));
	testSet.push_back(string("aaa"));
	testSet.push_back(string("abcba"));
	testSet.push_back(string("abba"));

	for (string test : testSet) {
		cout << "'" << test << "' -> " << solution.countSubstrings(test) << endl;
	}
}
