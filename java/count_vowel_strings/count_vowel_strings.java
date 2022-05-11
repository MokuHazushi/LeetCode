// https://leetcode.com/problems/count-sorted-vowel-strings/

package count_vowel_strings;

class Solution {
    public int countVowelStrings(int n) {
        int[] memory = {1,1,1,1,1};
		int ans = 0;

		while (n > 1) {
			for (int i=3; i>=0; i--) {
				memory[i] = memory[i+1] + memory[i];
			}
			n--;
		}

		for (int val : memory)
			ans += val;

		return ans;
    }
}

class Main {
	public static void main(String args[]) {
		System.out.println("Counting vowel strings.");
		Solution solution = new Solution();

		for (int i=1; i<=5; i++)
			System.out.println("n="+i+" -> "+solution.countVowelStrings(i));
	}
}
