#include <iostream>
#include <vector>
#include <string>
using namespace std;


class Solution {
public:
    bool isPalindrome(int x) {
        if (x == 0)
            return true;
        if (x < 0 || x % 10 == 0)
            return false;
        
        int reversedNumber = 0;
        int digit;
        do {
            digit = x % 10; 
            reversedNumber *= 10;
            reversedNumber += digit;
            x = x/10;   
        }
        while(x > reversedNumber);
        
        return (x - reversedNumber == 0) || ((x*10)+digit - reversedNumber == 0);
    }
};

string boolcast(bool b) {
    if (b)
        return string("TRUE");
    return string("FALSE");
}

int main() {
    Solution solution;
    vector<int> testSet = {12321, 1, 123321, -12321, 123};

    cout << "Is number n a palindrome?" << endl;
    for (int n : testSet) {
        cout << "n=" << n << " ->" << boolcast(solution.isPalindrome(n)) << endl;
    }

}