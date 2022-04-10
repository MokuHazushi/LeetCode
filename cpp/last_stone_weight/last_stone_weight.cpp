#include <iostream>
#include <my_utils.h>
#include <vector>
#include <algorithm>

class Solution {
public:
    int lastStoneWeight(vector<int>& stones) {
        make_heap(stones.begin(), stones.end());
        
        while (stones.size() > 1) {
            int first, second;
            first = stones[0];
            pop_heap(stones.begin(), stones.end());
            stones.pop_back();
            second = stones[0];
            pop_heap(stones.begin(), stones.end());
            stones.pop_back();
            
            if (first == second)
                continue;
            if (first < second)
                stones.push_back(second-first);
            else
                stones.push_back(first-second);
            push_heap(stones.begin(), stones.end());   
        }
        if (stones.empty())
            return 0;
        return stones[0];
    }    
};

int main() {
	Solution solution;
	vector<vector<int>> testSet;

	testSet.push_back(vector<int>{2,7,4,1,8,1});
	testSet.push_back(vector<int>{1});
	testSet.push_back(vector<int>{1,1,1,1});
	testSet.push_back(vector<int>{7,5,6,9,10,5});

	cout << "Last stone weight:" << endl;
	for (vector<int> test : testSet) {
		cout << NumberVectorToString(test) << " -> " << solution.lastStoneWeight(test) << endl;
	}
}
