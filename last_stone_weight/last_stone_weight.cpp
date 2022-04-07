#include <iostream>
#include <my_utils.h>
#include <vector>
#include <algorithm>

class Solution {
public:
    int lastStoneWeight(vector<int>& stones) {
        pair<int, int> max;
        
        while (stones.size() > 1) {
            max = findMaximums(stones);
			cout << "max.first=" << max.first << " max.second=" << max.second << endl;  
            if (stones[max.first] == stones[max.second]) {
                stones.erase(stones.begin()+max.second);
                stones.erase(stones.begin()+max.first);
            }
            else if (stones[max.first] > stones[max.second]) {
                stones[max.first] -= stones[max.second];
                stones.erase(stones.begin()+max.second);
            }
            else {
                stones[max.second] -= stones[max.first];
                stones.erase(stones.begin()+max.first);
            }
        }
        if (stones.empty())
            return 0;
        return stones[0]; 
    }
    
    pair<int,int> findMaximums(vector<int>& v) {
        int max1, max2;
        
        max1 = 0;
        max2 = 1;
        for (int i=2; i<v.size(); i++) {
            if (v[i] > v[max1]) {
				if (v[max1] > v[max2])
					max2 = max1;
                max1 = i;
			}
            else if (v[i] > v[max2])
                max2 = i;
        }
        if (max2 > max1)
            return pair<int, int>(max1, max2);
        return pair<int, int>(max2, max1);
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
