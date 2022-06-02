// https://leetcode.com/problems/minimum-knight-moves/
package min_knight_moves;

import java.util.Arrays;
import java.util.List;
import java.util.ArrayList;

import java.util.Queue;
import java.util.ArrayDeque;
import java.util.Map;
import java.util.HashMap;

class Position {
	int x, y;

	public Position(int x, int y) {
		this.x = x;
		this.y = y;
	}

	@Override
	public boolean equals(Object o) {
		if (o instanceof Position) {
			Position other = (Position)o;
			return other.x == x && other.y == y;
		}
		return false;
	}

	@Override
	public int hashCode() {
		int hash = 7;
		hash = 31 * hash + Integer.hashCode(x);
		hash = 31 * hash + Integer.hashCode(y);
		return hash;
	}
}

class Move {
	Position pos;
	int[] direction;

	public Move(Position pos, int[] direction) {
		this.pos = pos;
		this.direction = direction;
	}

	public boolean inBounds() {
		return (pos.x > 0 || pos.y > 0) && (direction[0] > 0 || direction[1] > 0);
	}

	public void updateMemory(Map<Position, Integer> memory) {
		Position comeFrom = new Position(pos.x - direction[0], pos.y - direction[1]);
		memory.put(pos, memory.get(comeFrom)+1);
	}
}

class Solution {
    public int minKnightMoves(int x, int y) {

		if (x == 0 && y == 0)
			return 0;

		// In clock-wise order
        int[][] allDirections = new int[][] {
			{1,2}, {2,1}, {2,-1}, {1,-2}, {-2, 1}, {-1, 2}
		};

		Map<Position, Integer> numberOfMoves = new HashMap<>();
		numberOfMoves.put(new Position(0, 0), 0);

		Queue<Move> nextPossibleMoves = new ArrayDeque<>();
		for (int[] direction : allDirections) {
			Move move = new Move(new Position(direction[0], direction[1]), direction);
			nextPossibleMoves.add(move);
			move.updateMemory(numberOfMoves);
		}

		while (!nextPossibleMoves.isEmpty()) {
			Move nextMove = nextPossibleMoves.poll();
			Position pos = nextMove.pos;

			if (pos.x == Math.abs(x) && pos.y == Math.abs(y))
				return numberOfMoves.get(pos);
			
			for (int[] direction : allDirections) {
				Position nextPos = new Position(pos.x + direction[0], pos.y + direction[1]);
				Move move = new Move(nextPos, direction);
				if (move.inBounds() && !numberOfMoves.containsKey(nextPos))
					nextPossibleMoves.add(move);
					move.updateMemory(numberOfMoves);
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

		testSet.add(new int[]{0,1});
		testSet.add(new int[]{2,1});
		testSet.add(new int[]{5,5});
		testSet.add(new int[]{-150, -150});
		testSet.add(new int[]{209, -58});
		testSet.add(new int[]{105, 100});

		for (int[] test : testSet)
			System.out.println(Arrays.toString(test) + " -> " + solution.minKnightMoves(test[0], test[1]));
	}
}
