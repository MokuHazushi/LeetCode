#include <iostream>
#include <tgmath.h>
#include <vector>
using namespace std;

class Solution {
public:
    int reverse(int x) {
        bool isNegative;
        int MAX_INT_10 = (pow(2, 31)-1)/10;

        
        if (x == 0)
            return 0;
        
        isNegative = x < 0;
        
        int result, digit;
        result = 0;
        x = abs(x);
        while (x > 0) {
            digit = x % 10;
            x /= 10;
            
            if (result < MAX_INT_10)
                result = result*10 + digit;
            else if (result == MAX_INT_10) {
                if (digit <= 7)
                    result = result*10 + digit;
                else
                    return 0;
            }
            else
                return 0;
        }
        
        if (isNegative)
            return -result;
        return result;
    }
};


int main() {
    Solution solution;
    vector<int> testSet {123, -123, 5, -5, -63, 0, 1463847412};

    cout << "Reverse integer:" << endl;
    for (int n : testSet) {
        cout << n << "->" << solution.reverse(n) << endl;
    }
}