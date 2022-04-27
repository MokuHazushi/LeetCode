// https://leetcode.com/problems/smallest-string-with-swaps/

#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <stack>
#include <my_utils.h>
#include <algorithm>

struct ConnectedEls {
	vector<int> connectedIndexes;
	vector<char> connectedChars;

	ConnectedEls(vector<int> is, vector<char> cs) : connectedIndexes(is), connectedChars(cs) {}
};

class Solution {
public:
    string smallestStringWithSwaps(string s, vector<vector<int>>& pairs) {
		vector<vector<int>> adjency;
		vector<bool> visited(s.length(), false);

		adjency = createAdjencyMap(pairs, s.length());
		for (int i=0; i<s.length(); i++) {
			if (visited[i])
				continue;

			ConnectedEls els = dfs(i, adjency, visited, s);
			sort(els.connectedIndexes.begin(), els.connectedIndexes.end());
			sort(els.connectedChars.begin(), els.connectedChars.end());

			for (int i=0; i<els.connectedIndexes.size(); i++)
				s[els.connectedIndexes[i]] = els.connectedChars[i];
		}

		return s;
    }

	vector<vector<int>> createAdjencyMap(vector<vector<int>> &pairs, int size) {
		vector<vector<int>> adjency(size);
		for (vector<int> pair : pairs) {
			if (pair[0] == pair[1])
				continue;

			adjency[pair[0]].push_back(pair[1]);
			adjency[pair[1]].push_back(pair[0]);
		}
		return adjency;
	}

	ConnectedEls dfs(int root, vector<vector<int>> &adjency, vector<bool> &visited, string &s) {
		stack<int> search;
		vector<int> indexes;
		vector<char> chars;

		search.push(root);
		while (!search.empty()) {
			int nextIndex = search.top();
			search.pop();

			if (visited[nextIndex])
				continue;
			
			visited[nextIndex] = true;
			indexes.push_back(nextIndex);
			chars.push_back(s[nextIndex]);
			for (int i : adjency[nextIndex]) {
                if (!visited[i])
                    search.push(i);
            }
		}
		return ConnectedEls(indexes, chars);
	}
};

struct Test {
	string s;
	vector<vector<int>> pairs;

	Test(string s, vector<vector<int>> pairs) : s(s), pairs(pairs) {}
};

int main() {
	Solution solution;
	vector<Test> testSet;

	testSet.push_back(Test("dcab", 
	vector<vector<int>>({
		vector<int>({0,3}),
		vector<int>({1,2})})));

	testSet.push_back(Test("dcab", 
	vector<vector<int>>({
		vector<int>({0,3}),
		vector<int>({1,2}),
		vector<int>({0,2})})));

	testSet.push_back(Test("cba", 
	vector<vector<int>>({
		vector<int>({0,1}),
		vector<int>({1,2})})));

	testSet.push_back(Test("dcad", 
	vector<vector<int>>({
		vector<int>({0,3}),
		vector<int>({1,2}),
		vector<int>({0,2})})));
	testSet.push_back(Test("udyyek", 
	vector<vector<int>>({
		vector<int>({3,3}),
		vector<int>({3,0}),
		vector<int>({5,1}),
		vector<int>({3,1}),
		vector<int>({3,4}),
		vector<int>({3,5})})));

	for (Test test : testSet) {
		cout << "s='" << test.s << "', pairs=[";
		for (vector<int> pair : test.pairs)
			cout << NumberVectorToString(pair) << ",";
		cout << "]" << endl;
		cout << "\t-> " << solution.smallestStringWithSwaps(test.s, test.pairs) << endl;
	}
}
