#include <iostream>
#include <my_utils.h>
#include <vector>

struct Change {
	int i, j, value;
	Change(int i, int j, int value) : i(i), j(j), value(value) {}
};

class Solution {
public:
    void gameOfLife(vector<vector<int>>& board) {
		vector<Change> changes;

		for (int i=0; i<board.size(); i++) {
			for (int j=0; j<board[0].size(); j++) {
				vector<int> aliveNeighbors = findAliveNeighbors(i, j, board);

				if (board[i][j] == 1) {
					if (aliveNeighbors.size() < 2 || aliveNeighbors.size() > 3)
						changes.push_back(Change(i, j, 0));
				}
				else {
					if (aliveNeighbors.size() == 3)
						changes.push_back(Change(i, j, 1));
				}
			}
		}

		for (Change change : changes)
			board[change.i][change.j] = change.value;
    }

	vector<int> findAliveNeighbors(int i, int j, vector<vector<int>> &board) {
		vector<int> neighbors;
		for (int a=-1; a<=1; a++) {
			for (int b=-1; b<=1; b++) {
				if (a == 0 && b == 0)
					continue;
				
				if (!isInBoard(i+a, j+b, board.size(), board[0].size()))
					continue;

				if (board[i+a][j+b] == 1)
					neighbors.push_back(board[i+a][j+b]);
			}
		}

		return neighbors;
	}

	bool isInBoard(int i, int j, int dimX, int dimY) {
		if (i<0 || j<0)
			return false;
		if (i>=dimX || j>=dimY)
			return false;
		
		return true;
	}
};

int main() {
	Solution solution;
	vector<vector<vector<int>>> testSet;

	testSet.push_back(vector<vector<int>>({
		vector<int>({1,1}),
		vector<int>({1,0})
		}));
	
	cout << "Simulate one round of game of life:" << endl;
	for (vector<vector<int>> test : testSet) {
		cout << "Initial grid:\n" << DoubleNumberVectorToString(test) << endl;
		cout << "Result:\n" << endl;
		solution.gameOfLife(test);
		cout << DoubleNumberVectorToString(test) << endl;
	}
}
