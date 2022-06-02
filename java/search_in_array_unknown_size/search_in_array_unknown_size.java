// https://leetcode.com/problems/search-in-a-sorted-array-of-unknown-size/

package search_in_array_unknown_size;

import java.util.List;
import java.util.ArrayList;
import java.util.Arrays;

class ArrayReader {
	int[] sortedArray;

	public ArrayReader(int[] sortedArray) {
		this.sortedArray = sortedArray;
	}

	public int get(int index) {
		if (index >= sortedArray.length)
			return Integer.MAX_VALUE;
		return sortedArray[index];
	}
}

class Solution {
    public int search(ArrayReader reader, int target) {
		int left = 0;
		int right = 1;

		// if target is greater than all values in array, left = right = out of array bounds
		while (left <= right && reader.get(left) != Integer.MAX_VALUE) {
			int rightValue = reader.get(right);

			// Right boundary is too small, expand boundaries
			if (rightValue != Integer.MAX_VALUE && rightValue < target) {
				right = right << 1;
				continue;
			}
			
			int mid = left + (right-left)/2;
			int midValue = reader.get(mid);

			if (midValue == Integer.MAX_VALUE)
				right = mid;
			else {
				if (midValue == target)
					return mid;
				else if (midValue < target)
					left = mid+1;
				else
					right = mid-1;
			}
		}

		return -1;        
    }
}

class Test {
	ArrayReader reader;
	int target;

	public Test(ArrayReader reader, int target) {
		this.reader = reader;
		this.target = target;
	}
}

class Main {
	public static void main(String args[]) {
		System.out.println("Search target in an sorted array of unknown size");
		Solution solution = new Solution();
		List<Test> testSet = new ArrayList<>();

		testSet.add(new Test(
			new ArrayReader(new int[]{-1,0,3,5,9,12}), 
			9));
		testSet.add(new Test(
			new ArrayReader(new int[]{-1,0,3,5,9,12}), 
			2));

		testSet.add(new Test(
			new ArrayReader(new int[]{-1,0,3,5,9,12}), 
			13));

			testSet.add(new Test(
				new ArrayReader(new int[]{-1,0,3,5,9,12}), 
				-2));

		for (Test test : testSet) {
			System.out.println("secret=" + Arrays.toString(test.reader.sortedArray) + " target=" + test.target);
			System.out.println("\t-> " + solution.search(test.reader, test.target));
		}

	}
}
