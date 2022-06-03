// https://leetcode.com/problems/range-sum-query-2d-immutable/
package range_sum_query;

import java.util.List;
import java.util.ArrayList;

class NumMatrix {
	/*
	 * sumMatrix[i][j] is the sum of all the elements in range
	 * matrix[0][0] to matrix[i][j] (equivalent of sumRegion(0, 0, i, j))
	 */
	private int sumMatrix[][];

    public NumMatrix(int[][] matrix) {
		sumMatrix = new int[matrix.length][matrix[0].length];
		for (int i=0; i<matrix.length; i++) {
			int rowSum = 0;
			for (int j=0; j<matrix[0].length; j++) {
				sumMatrix[i][j] = matrix[i][j] + rowSum;
				rowSum += matrix[i][j];
				if (i > 0)
					sumMatrix[i][j] += sumMatrix[i-1][j];
			}
		}
    }
    
    public int sumRegion(int row1, int col1, int row2, int col2) {
		if (row1 < 0 || col1 < 0 || row2 < 0 || col2 < 0)
			return 0;
		
		// Recursive call prevents going out-of-bounds
		return sumMatrix[row2][col2] - 
			sumRegion(0, 0, row2, col1-1) -
			sumRegion(0, 0, row1-1, col2) + 
			sumRegion(0, 0, row1-1, col1-1);
    }
}

class Test {
	int row1, col1, row2, col2, expected;

	public Test(int row1, int col1, int row2, int col2, int expected) {
		this.row1 = row1;
		this.col1 = col1;
		this.row2 = row2;
		this.col2 = col2;
		this.expected = expected;
	}

	@Override
	public String toString() {
		return "{row1="+row1+" col1="+col1+" row2="+row2+" col2="+col2+" expected="+expected+"}";
	}
}


class Main {
	public static void main(String args[]) {
		System.out.println("Implement a NumMatrix class");

		int[][] mat1 = new int[][] {
			{1,2,3},
			{-1,0,-1},
			{1,1,1}
		};

		int[][] mat2 = new int[][] {
			{3,0,1,4,2},
			{5,6,3,2,1},
			{1,2,0,1,5},
			{4,1,0,1,7},
			{1,0,3,0,5}
		};

		System.out.println("Test on mat1");
		NumMatrix numMat1 = new NumMatrix(mat1);
		List<Test> testSet1 = new ArrayList<>();
		testSet1.add(new Test(0,0,2,2,7));
		testSet1.add(new Test(1,1,2,2,1));
		for (Test test : testSet1)
			System.out.println(test.toString()+" -> "+numMat1.sumRegion(test.row1, test.col1, test.row2, test.col2));

	}
}
