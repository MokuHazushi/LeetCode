#include <iostream>
#include <my_utils.h>

class Solution {
public:
    int maxArea(vector<int>& height) {
		int i, j, area;

		i = 0;
		j = height.size()-1;
		area = computeArea(height, i, j);

		if (height[i] < height[j])
			i++;
		else 
			j--;
		
		while (i < j) {
			area = max(area, computeArea(height, i, j));

			if (height[i] < height[j])
				i++;
						
			else
				j--;
		}

		return area;
	}

	int computeArea(vector<int>& height, int i, int j) {
		return (j-i) * min(height[i], height[j]);
	}
};

int main() {
	Solution solution;
	vector<vector<int>> testSet;

	testSet.push_back(vector<int>{1,1});
	testSet.push_back(vector<int>{1,9,9,1});
	testSet.push_back(vector<int>{1,9,1,1,1,9});
	testSet.push_back(vector<int>{1,2,2,1,1,1,1,1});
	testSet.push_back(vector<int>{1,8,6,2,5,4,8,3,7});

	cout << "Container with most water contains :" << endl;
	for (vector<int> v : testSet) {
		cout << NumberVectorToString(v) << " -> " << solution.maxArea(v) << endl;
	}
}
