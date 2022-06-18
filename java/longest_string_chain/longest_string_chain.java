// https://leetcode.com/problems/longest-string-chain/

package longest_string_chain;

import java.util.List;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;

class Solution {
    public int longestStrChain(String[] words) {
		int memory[] = new int[words.length];
		int ans = 0;

		Arrays.sort(words, new Comparator<String>() {
			@Override
			public int compare(String s1, String s2) {
				return Integer.compare(s1.length(), s2.length());
			}
		});

		for (int i=0; i<words.length; i++) {
			for (int j=i+1; j<words.length; j++) {
				if (isPredecessor(words[i], words[j]))
					memory[j] = Math.max(memory[j], memory[i]+1);
			}
		}

		for (int chain : memory)
			ans = Math.max(ans, chain);
		return ans+1;
    }

	// return true if word1 is predecessor of word2
	// linear time
	boolean isPredecessor(String word1, String word2) {
		if (word2.length() - word1.length() != 1)
			return false;

		int i, j;
		boolean skippedOne;

		i = j = 0;
		skippedOne = false;

		while (i < word1.length()) {
			if (word1.charAt(i) == word2.charAt(j)) {
				i++;
				j++;
				continue;
			}

			if (skippedOne)
				return false;

			skippedOne = true;
			j++;
		}

		return true;
	}
}

class Main {
	public static void main(String args[]) {
		System.out.println("Find longest string chain");
		Solution solution = new Solution();

		List<String[]> testSet = new ArrayList<>();

		testSet.add(new String[]{"a", "b", "ba", "bca", "bda", "bdca"});
		testSet.add(new String[]{"xbc", "pcxbcf", "xb", "cxbc", "pcxbc"});
		testSet.add(new String[]{"abcd", "dbqca"});
		testSet.add(new String[]{"a", "b", "ab", "bac"});

		for (String[] test : testSet) {
			System.out.println(Arrays.toString(test));
			System.out.println("-> " + solution.longestStrChain(test));
		}
	}
}
