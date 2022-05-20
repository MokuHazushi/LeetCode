package unique_paths_ii;

import java.util.List;
import java.util.ArrayList;
import java.util.Arrays;

class Solution {
    public int uniquePathsWithObstacles(int[][] obstacleGrid) {

		if (obstacleGrid[0][0] == 1 ||
			obstacleGrid[obstacleGrid.length-1][obstacleGrid[0].length-1] == 1)
			return 0;

		int[][] memory = new int[obstacleGrid.length][obstacleGrid[0].length];
		return dp_topdown(0, 0, obstacleGrid, memory);		        
    }

	int dp_topdown(int i, int j, int[][] obstacleGrid, int[][] memory) {
		if (i == obstacleGrid.length-1 && j == obstacleGrid[0].length-1)
			return 1;
		
		if (memory[i][j] == 0) {
			if (i+1 < obstacleGrid.length && obstacleGrid[i+1][j] == 0)
				memory[i][j] += dp_topdown(i+1, j, obstacleGrid, memory);
			if (j+1 < obstacleGrid[0].length && obstacleGrid[i][j+1] == 0)
				memory[i][j] += dp_topdown(i, j+1, obstacleGrid, memory);
		}

		return memory[i][j];
	}
}

class Main {
	public static void main(String args[]) {
		System.out.println("Count number of unique paths");
		Solution solution = new Solution();
		List<int[][]> testSet = new ArrayList<>();

		testSet.add(new int[][]{
			{0,0,0},
			{0,1,0},
			{0,0,0}
		});

		testSet.add(new int[][]{
			{1,0}
		});

		for (int[][] test : testSet) {
			System.out.println("Grid=");
			for (int i=0; i<test.length; i++) {
				System.out.println("\t"+Arrays.toString(test[i]));
			}
			System.out.println("-> "+solution.uniquePathsWithObstacles(test));
		}
	}
}
