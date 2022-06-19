// https://leetcode.com/problems/search-suggestions-system/

package search_suggestions_system;

import java.util.List;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Deque;
import java.util.ArrayDeque;

class SearchEl {
	String suffix;
	Dictionary iter;

	public SearchEl(String suffix, Dictionary iter) {
		this.suffix = suffix;
		this.iter = iter;
	}
}

class Dictionary {
	Dictionary[] letters = new Dictionary[26];
	boolean isWord = false;
	static int shift = 97; // ASCII table 'a'

	Dictionary dictionaryAt(char letter) {
		return letters[letter - shift];
	}

	void createDictionaryAt(char letter) {
		letters[letter - shift] = new Dictionary();
	}

	// Maximum of 3 words, in lexicographic order
	List<String> findWords(String prefix) {
		List<String> words = new ArrayList<>();
		Dictionary iter = this;
		Deque<SearchEl> dfs = new ArrayDeque<>();

		for (char letter : prefix.toCharArray()) {
			iter = iter.dictionaryAt(letter);
			if (iter == null)
				return new ArrayList<>();
		}

		dfs.addFirst(new SearchEl("", iter));
		while (!dfs.isEmpty() && words.size() < 3) {
			SearchEl el = dfs.removeFirst();

			if (el.iter.isWord)
				words.add(prefix + el.suffix);

			for (int i = el.iter.letters.length-1; i >= 0; i--) {
				if (el.iter.letters[i] != null) {
					dfs.addFirst(new SearchEl(el.suffix + (char)('a' + i), el.iter.letters[i]));
				}
			}
		}
		return words;
	}
}

class Solution {
	public List<List<String>> suggestedProducts(String[] products, String searchWord) {
		Dictionary dic = new Dictionary();
		List<List<String>> searchResults = new ArrayList<>();
		String prefix = "";

		for (String product : products) {
			Dictionary iter = dic;
			for (char letter : product.toCharArray()) {
				if (iter.dictionaryAt(letter) == null)
					iter.createDictionaryAt(letter);
				iter = iter.dictionaryAt(letter);
			}
			iter.isWord = true;
		}

		for (char letter : searchWord.toCharArray()) {
			prefix += letter;
			searchResults.add(dic.findWords(prefix));
		}

		return searchResults;
	}
}

class Test {
	String[] products;
	String searchWord;

	public Test(String[] products, String searchWord) {
		this.products = products;
		this.searchWord = searchWord;
	}
}

class Main {
	public static void main(String args[]) {
		System.out.println("Give suggestions for prefix of all size");
		Solution solution = new Solution();

		List<Test> testSet = new ArrayList<>();

		testSet.add(new Test(new String[]{"mobile", "mouse", "moneypot", "monitor", "mousepad"}, "mouse"));
		testSet.add(new Test(new String[]{"havana"}, "tatiana"));

		for (Test test : testSet) {
			System.out.println(Arrays.toString(test.products) + " search=" + test.searchWord);
			System.out.println("\t-> ");
			List<List<String>> result = solution.suggestedProducts(test.products, test.searchWord);
			for (List<String> words : result)
				System.out.println("\t" + Arrays.toString(words.toArray()));
		}
	}
}
