// https://leetcode.com/problems/coin-change/

package coin_change;

import java.util.List;
import java.util.ArrayList;
import java.util.Arrays;

class Solution {
    public int coinChange(int[] coins, int amount) {
        int[] memory = new int[amount+1];

		//return dp_topdown(coins, amount, memory);
		return dp_bottomup(coins, amount, memory);
    }

	int dp_topdown(int[] coins, int remainingAmount, int[] memory) {
		if (remainingAmount == 0)
			return 0;
		
		if (memory[remainingAmount] == 0) {
			int minVal = Integer.MAX_VALUE;
			for (int coin : coins) {
				if (remainingAmount-coin < 0)
					continue;

				int val = 1+dp_topdown(coins, remainingAmount-coin, memory);
				
				if (val > 0)
					minVal = Math.min(minVal, val);
			}

			if (minVal == Integer.MAX_VALUE)
				memory[remainingAmount] = -1;
			else
				memory[remainingAmount] = minVal;
		}

		return memory[remainingAmount];
	}

	int dp_bottomup(int[] coins, int amount, int[] memory) {

		for (int i=1; i<=amount; i++) {
			int minVal = Integer.MAX_VALUE;
			for (int coin : coins) {
				if (i-coin >= 0 && memory[i-coin] >= 0)
					minVal = Math.min(minVal, memory[i-coin]);
			}

			if (minVal == Integer.MAX_VALUE)
				memory[i] = -1;
			else
				memory[i] = 1+minVal;
		}
		return memory[amount];
	}
}

class Test {
	int[] coins;
	int amount;

	public Test(int[] coins, int amount) {
		this.coins = coins;
		this.amount = amount;
	}
}

class Main {
	public static void main(String args[]) {
		System.out.println("Find fewest number of coin to get amount");
		Solution solution = new Solution();
		List<Test> testSet = new ArrayList<>();

		testSet.add(new Test(new int[]{1,2,5}, 11));
		testSet.add(new Test(new int[]{2}, 3));
		testSet.add(new Test(new int[]{1}, 0));

		for (Test test : testSet) {
			System.out.println("coins=" + Arrays.toString(test.coins) + " amount=" + test.amount);
			System.out.println("\t-> " + solution.coinChange(test.coins, test.amount));
		}
	}
}
