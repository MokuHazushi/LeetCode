// https://leetcode.com/problems/max-area-of-island/

package max_area_island;

import java.util.List;
import java.util.ArrayList;
import java.util.Stack;

class Solution {
    public int maxAreaOfIsland(int[][] grid) {
		int maxArea = 0;
		boolean[][] visited = new boolean[grid.length][grid[0].length];

		for (int i=0; i<grid.length; i++) {
			for (int j=0; j<grid[0].length; j++) {
				if (visited[i][j] || grid[i][j] == 0)
					continue;
				maxArea = Integer.max(maxArea, islandSize(grid, visited, i, j));
			}
		}
		return maxArea;        
    }

	public int islandSize(int[][] grid, boolean[][] visited, int i, int j) {
		int size = 0;
		int dimX = grid.length-1;
		int dimY = grid[0].length-1;
		Stack<int[]> stack = new Stack<>();

		stack.push(new int[]{i, j});
		while (!stack.empty()) {
			int[] node = stack.pop();
			size++;
			visited[node[0]][node[1]] = true;

			for (int[] neighbor : neighbors(node[0], node[1], dimX, dimY)) {
				if (!visited[neighbor[0]][neighbor[1]] && grid[neighbor[0]][neighbor[1]] == 1) {
					stack.push(neighbor);
					visited[neighbor[0]][neighbor[1]] = true;
				}
			}
		}

		return size;
	}

	public List<int[]> neighbors(int i, int j, int dimX, int dimY) {
		int[][] allDirections = new int[][] {
			{i, j-1}, {i, j+1}, {i-1, j}, {i+1, j}
		};
		List<int[]> neighbors = new ArrayList<>();
		for (int[] direction : allDirections) {
			if (inBounds(direction[0], direction[1], dimX, dimY))
				neighbors.add(direction);
		}
		return neighbors;
	}

	public boolean inBounds(int i, int j, int dimX, int dimY) {
		return i >= 0 && i <= dimX && j >= 0 && j <= dimY;
	}
}

class Main {
	public static void main(String args[]) {
		System.out.println("Find largest island");
		Solution solution = new Solution();
		List<int[][]> testSet = new ArrayList<>();

		testSet.add(new int[][]{
			{0,0,1,0,0,0,0},
			{1,0,0,0,0,1,1},
			{1,1,1,0,0,0,1},
			{0,0,0,0,0,0,0}
		});

		testSet.add(new int[][]{
			{1,1,1,1,1},
			{1,0,0,0,1},
			{1,1,1,1,1}
		});

		int testNumber = 1;
		for (int[][] test : testSet) {
			System.out.println("Test #" + testNumber + " -> " + solution.maxAreaOfIsland(test));
			testNumber++;
		}
	}
}
