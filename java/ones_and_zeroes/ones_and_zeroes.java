// https://leetcode.com/problems/m-and-n/

package ones_and_zeroes;

import java.util.List;
import java.util.ArrayList;
import java.util.Arrays;

class OnesAndZeroes {
	int m, n;

	public OnesAndZeroes(int m, int n) {
		this.m = m;
		this.n = n;
	}
}

class Solution {
    public int findMaxForm(String[] strs, int m, int n) {
		List<OnesAndZeroes> data = new ArrayList<>();
		int[][] memory = new int[m+1][n+1];

		for (String s : strs) {
			int zeroes = 0;
			int ones = 0;
			for (char c : s.toCharArray()) {
				if (c == '0')
					zeroes++;
				else
					ones++;
			}
			data.add(new OnesAndZeroes(zeroes, ones));
		}
		
		for (int i=0; i<data.size(); i++) {
			int zeroes = data.get(i).m;
			int ones = data.get(i).n;
			for (int j=m; j>=zeroes; j--) {
				for (int k=n; k>=ones; k--) {
					memory[j][k] = Math.max(1+memory[j-zeroes][k-ones], memory[j][k]);
				}
			}
		}

		return memory[m][n];
    }
}

class Test {
	String[] strs;
	int m, n;

	public Test(String[] strs, int m, int n) {
		this.strs = strs;
		this.m = m;
		this.n = n;
	}
}

class Main {
	public static void main(String args[]) {
		System.out.println("Count largest subset");
		Solution solution = new Solution();
		List<Test> testSet = new ArrayList<>();

		testSet.add(new Test(
			new String[]{"11111", "100", "1101", "1101", "11000"}, 
			5, 7));

		testSet.add(new Test(
			new String[]{"001", "110", "0000", "0000"}, 
			9, 2));

		testSet.add(new Test(
			new String[]{"10", "0", "1"}, 
			1, 1));


		testSet.add(new Test(
			new String[]{"10", "0001", "111001", "1", "0"}, 
			5, 3));


		for (Test test : testSet) {
			System.out.println(Arrays.toString(test.strs) + " m=" + test.m + " n=" + test.n);
			System.out.println("\t-> " + solution.findMaxForm(test.strs, test.m, test.n));
		}
	}
}
