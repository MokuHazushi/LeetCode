// https://leetcode.com/problems/triangle/

package triangle;

import java.util.List;
import java.util.ArrayList;

class Solution {
    public int minimumTotal(List<List<Integer>> triangle) {
		for (int row=1; row<triangle.size(); row++) {
			int rowSize = triangle.get(row).size();
			for (int colum=0; colum<rowSize; colum++) {
				int curNum = triangle.get(row).get(colum);
				if (colum == 0)
					triangle.get(row).set(colum, triangle.get(row-1).get(0)+curNum);
				else if (colum == rowSize-1)
					triangle.get(row).set(colum, triangle.get(row-1).get(rowSize-2)+curNum);
				else {
					int min = Math.min(triangle.get(row-1).get(colum-1), triangle.get(row-1).get(colum));
					triangle.get(row).set(colum, curNum+min);
				}
			}
		}

		return triangle.get(triangle.size()-1).stream().min(Integer::compare).get();
    }
}

class Main {
	public static void main(String args[]) {
		System.out.println("Triangle");
		Solution solution = new Solution();

		List<List<Integer>> t1 = new ArrayList<>();
		List<Integer> t11 = new ArrayList<>();
		List<Integer> t12 = new ArrayList<>();
		List<Integer> t13 = new ArrayList<>();
		List<Integer> t14 = new ArrayList<>();
		t11.add(2);
		t12.add(3); t12.add(4);
		t13.add(6); t13.add(5); t13.add(7);
		t14.add(4); t14.add(1); t14.add(8); t14.add(3);

		t1.add(t11); 
		t1.add(t12); 
		t1.add(t13); 
		t1.add(t14);
		System.out.println("t1 -> " + solution.minimumTotal(t1));
	}
}
