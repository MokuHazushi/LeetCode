#include <iostream>
#include <vector>
#include <string>
#include <my_utils.h>

class Solution {
public:
    string longestPalindrome(string s) {
        int length = s.size()-1;
        
        while (length >= 1) {
            int i=0;
            while (i+length < s.size()) {
                if (isPalindrome(s, i, i+length))
                    return s.substr(i, length+1);
                i++;
            }
            length--;
        }
        
        return string(1, s[0]);   
    }
    
    bool isPalindrome(string &s, int i, int j) {
		cout << "Is palindrome between i=" << i << " j=" << j << endl;

        while (i < j) {
            if (s[i] != s[j])
                return false;
            i++;
            j--;
        }
        return true;
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
