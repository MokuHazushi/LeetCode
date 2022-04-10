#include <iostream>
#include <string>
#include <vector>
#include <cctype>
#include <my_utils.h>

class Solution {
	public:
	bool isPalindrome(string s) {
			string cleanText;
			int i,j;
			
			for (char c : s) {
				if (isalnum(c)) {
					if (isalpha(c))
						cleanText.push_back(tolower(c));
					else
						cleanText.push_back(c);
				}
			}
			
			i=0;
			j=cleanText.size()-1;
			while (i<j) {
				if (cleanText[i] != cleanText[j])
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

	testSet.push_back("A man, a plan, a canal: Panama");
	testSet.push_back("A MAN, A PLAN, A CANAL: PANAMA");
	testSet.push_back(" ");
	testSet.push_back("1234321");
	testSet.push_back("1aa1");
	testSet.push_back("Hello Word!");

	cout << "I following setence a palindrome?" << endl;
	for (string s : testSet) {
		cout << "'" << s << "' -> " << booleanToString(solution.isPalindrome(s)) << endl;
	}
	
}
