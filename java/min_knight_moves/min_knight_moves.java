// https://leetcode.com/problems/minimum-knight-moves/
package min_knight_moves;

import java.util.Arrays;
import java.util.List;
import java.util.ArrayList;

import java.util.Queue;
import java.util.ArrayDeque;

class Move {
	int x, y;
	int[] direction;

	public Move(int x, int y, int[] direction) {
		this.x = x;
		this.y = y;
		this.direction = direction;
	}

	public boolean inBounds() {
		return (x >= 0 && y >= 0) && !(x == 0 && y == 0);
	}

	public void updateMemory(int[][] memory) {
		int[] comeFrom = new int[]{x - direction[0], y - direction[1]};
		memory[x][y] = 1 + memory[comeFrom[0]][comeFrom[1]];
	}
}

class Solution {
    public int minKnightMoves(int x, int y) {

		if (x == 0 && y == 0)
			return 0;
		
		if (Math.abs(x) == 1 && Math.abs(y) == 1)
			return 2;

		// In clock-wise order
        int[][] allDirections = new int[][] {
			{1,2}, {2,1}, {2,-1}, {1,-2}, {-1, -2}, {-2, -1}, {-2, 1}, {-1, 2}
		};

		int[][] numberOfMoves = new int[303][303];

		Queue<Move> nextPossibleMoves = new ArrayDeque<>();
		for (int[] direction : allDirections) {
			Move move = new Move(direction[0], direction[1], direction);
			if (move.inBounds()) {
				nextPossibleMoves.add(move);
				move.updateMemory(numberOfMoves);
			}
		}

		while (!nextPossibleMoves.isEmpty()) {
			Move nextMove = nextPossibleMoves.poll();

			if (nextMove.x == Math.abs(x) && nextMove.y == Math.abs(y))
				return numberOfMoves[nextMove.x][nextMove.y];
			
			for (int[] direction : allDirections) {
				int[] nextPos = new int[]{
					nextMove.x + direction[0], 
					nextMove.y + direction[1]};
				Move move = new Move(nextPos[0], nextPos[1], direction);
				if (move.inBounds() && numberOfMoves[nextPos[0]][nextPos[1]] == 0) {
					nextPossibleMoves.add(move);
					move.updateMemory(numberOfMoves);
				}
			}
		}

		return -1;
    }
}

class Main {
	public static void main(String args[]) {
		System.out.println("Find minimum knight moves");
		Solution solution = new Solution();
		List<int[]> testSet = new ArrayList<>();

		testSet.add(new int[]{0,2});
		testSet.add(new int[]{0,1});
		testSet.add(new int[]{2,1});
		testSet.add(new int[]{5,5});
		testSet.add(new int[]{-150, -150});
		testSet.add(new int[]{209, -58});
		testSet.add(new int[]{105, 100});
		testSet.add(new int[]{1,1});
		testSet.add(new int[]{300, 0});
		testSet.add(new int[]{-1, 1});

		for (int[] test : testSet)
			System.out.println(Arrays.toString(test) + " -> " + solution.minKnightMoves(test[0], test[1]));
	}
}
