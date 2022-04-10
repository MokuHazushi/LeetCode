#include <iostream>
#include <vector>
#include <string>
using namespace std;


class Solution {
public:
    bool validPalindrome(string s) {
        
        int wrongIndex = isPalindrome(s);
        
        if (wrongIndex == -1)
            return true;
        
        char save = s[wrongIndex];
        
        s.erase(wrongIndex, 1);
        if (isPalindrome(s) == -1)
            return true;
        
        s.insert(s.begin()+wrongIndex, save);
        s.erase(s.size()-wrongIndex-1, 1);
        return isPalindrome(s) == -1;
    }
    
    int isPalindrome(string s) {
        
        if (s.size() <= 1)
            return -1;
        
        int middle;
        if (s.size()%2 == 0)
            middle = s.size()/2;
        else
            middle = (s.size()/2)-1;
        
        for (int i=0; i<=middle; i++) {
            if (s[i] != s[s.size()-i-1])
                return i;
        }
        
        return -1;
    }
};

string boolcast(bool b) {
    if (b)
        return string("TRUE");
    return string("FALSE");
}

int main() {
    Solution solution;
    vector<string> testSet {"cbbcc", "aba", "a", "acba", "aacbb", "acabb", "aabcb"};

    cout << "Is string s a palindrome if we remove at most one character?" << endl;
    for (string s : testSet) {
        cout << "'" << s << "' ->" << boolcast(solution.validPalindrome(s)) << endl;
    }

}