package divide_two_integers;

import java.util.List;
import java.util.ArrayList;
import java.util.Stack;

class Solution {
    public int divide(int dividend, int divisor) {
		boolean oppositeSign = (dividend < 0 && divisor > 0) || (dividend > 0 && divisor < 0);
		int ans = 0;
		int absDivisor = Math.abs(divisor);
		Stack<Integer> divisorQueue = new Stack<>();
		Stack<Integer> powerQueue = new Stack<>();
		divisorQueue.push(absDivisor);
		powerQueue.push(1);

		if (divisor == Integer.MIN_VALUE)
			return dividend == Integer.MIN_VALUE ? 1 : 0;

		if (absDivisor == 1) {
			if (oppositeSign) {
				return dividend < 0 ? dividend : -dividend;
			}
			return dividend == Integer.MIN_VALUE ? Integer.MAX_VALUE : Math.abs(dividend);
		}

		while (!divisorQueue.isEmpty()) {
			int nextDivisor = divisorQueue.peek();
			int nextPower = powerQueue.peek();
			if (dividend > 0) {
				if (dividend < nextDivisor) {
					divisorQueue.pop();
					powerQueue.pop();
					continue;
				}
				dividend -= nextDivisor;
			}
			else {
				if (dividend > -nextDivisor) {
					divisorQueue.pop();
					powerQueue.pop();
					continue;
				}
				dividend += nextDivisor;
			}

			if (oppositeSign)
				ans -= nextPower;
			else
				ans += nextPower;

			if (nextDivisor < 1073741824) {
				divisorQueue.push(nextDivisor+nextDivisor);
				powerQueue.push(nextPower+nextPower);
			}
		}
		
		return ans;
    }
}

class Main {
	public static void main(String args[]) {
		System.out.println("Divide 2 integers w/o using multiplication or division");
		Solution solution = new Solution();

		List<int[]> testSet = new ArrayList<>();

		testSet.add(new int[]{10, 3});
		testSet.add(new int[]{10, -3});
		testSet.add(new int[]{-10, 3});
		testSet.add(new int[]{-10, -3});

		testSet.add(new int[]{300, 10});

		testSet.add(new int[]{-2147483648, -2147483648});
		testSet.add(new int[]{1, -2147483648});

		testSet.add(new int[]{-2147483648, 1});
		testSet.add(new int[]{-2147483648, -1});

		for (int[] test : testSet) {
			System.out.println("dividend=" + test[0] + " divisor=" + test[1] + " -> " + solution.divide(test[0], test[1]));
		}

		System.out.println("Time test");
		solution.divide(2147483647,2);
	}
}
