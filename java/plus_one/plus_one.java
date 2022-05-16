package plus_one;

// https://leetcode.com/problems/plus-one/

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

class Solution {
    public int[] plusOne(int[] digits) {
		int remainder = 1;
		int i = digits.length-1;
		while (remainder > 0 && i >= 0) {
			if (digits[i] == 9)
				digits[i] = 0;
			else {
				digits[i]++;
				remainder = 0;
			}
			i--;
		}

		if (remainder > 0) {
			int[] newDigits = new int[digits.length+1];
			newDigits[0] = 1;
			for (int k=0; k<digits.length; k++)
				newDigits[k+1] = digits[k];
			return newDigits;
		}
		return digits;        
    }
}

class Main {
	public static void main(String args[]) {
		System.out.println("Add one");
		Solution solution = new Solution();
		List<int[]> testSet = new ArrayList<>();

		testSet.add(new int[]{1});
		testSet.add(new int[]{9, 9});

		for (int[] test : testSet) {
			System.out.println("digits="+Arrays.toString(test));
			System.out.println("\t-> "+Arrays.toString(solution.plusOne(test)));
		}
	}
}
