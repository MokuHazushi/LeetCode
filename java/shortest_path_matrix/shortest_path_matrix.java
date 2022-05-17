// https://leetcode.com/problems/shortest-path-in-binary-matrix/

package shortest_path_matrix;

import java.util.List;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Queue;
import java.util.ArrayDeque;

class Solution {
    public int shortestPathBinaryMatrix(int[][] grid) {
		Queue<int[]> pathFinder = new ArrayDeque<>();

		if (grid[0][0] == 1 || grid[grid.length-1][grid[0].length-1] == 1)
			return -1;
		
		pathFinder.add(new int[]{0,0});
		grid[0][0] = 1;

		while (!pathFinder.isEmpty()) {
			int[] nextNode = pathFinder.poll();
			int distance = grid[nextNode[0]][nextNode[1]];

			if (nextNode[0] == grid.length-1 && nextNode[1] == grid[0].length-1)
				return distance;

			for (int[] neighbor : getNeighbors(nextNode, grid)) {
				pathFinder.add(neighbor);
				grid[neighbor[0]][neighbor[1]] = distance+1;
			}
		}
        return -1;
    }

	private List<int[]> getNeighbors(int[] pos, int[][] grid) {
		List<int[]> neighbors = new ArrayList<>();

		for (int i=pos[0]-1; i<=pos[0]+1; i++) {
			if (i < 0 || i >= grid.length)
				continue;
			for (int j=pos[1]-1; j<=pos[1]+1; j++) {
				if (i == pos[0] && j == pos[1])
					continue;
				
				if (j < 0 || j >= grid[0].length)
					continue;
				
				if (grid[i][j] == 0)
					neighbors.add(new int[]{i,j});
			}
		}

		return neighbors;
	}
}

class Main {
	public static void main(String args[]) {
		System.out.println("Find shortest path in binary matrix");
		Solution solution = new Solution();
		List<int[][]> testSet = new ArrayList<>();

		/*
		testSet.add(new int[][]{
			{0,1},
			{1,0}
		});

		testSet.add(new int[][]{
			{0,0,0},
			{1,1,0},
			{1,1,1}
		});
		*/

		testSet.add(new int[][]{
			{0,1,0,1,0},
			{1,0,0,0,1},
			{0,0,1,1,1},
			{0,0,0,0,0},
			{1,0,1,0,0}
		});

		for (int[][] test : testSet) {
			System.out.println("grid=");
			for (int[] tab : test)
				System.out.println("\t"+Arrays.toString(tab));
			System.out.println("-> "+solution.shortestPathBinaryMatrix(test));
		}
	}
}
