// https://leetcode.com/problems/letter-combinations-of-a-phone-number/

import java.util.Arrays;

import java.util.List;
import java.util.ArrayList;
import java.util.Hashtable;

class Solution {
    public List<String> letterCombinations(String digits) {
		Hashtable<Character, String[]> letters = new Hashtable<>();
		List<String> combinations = new ArrayList<>();

		if (digits.isEmpty())
			return combinations;

		letters.put('2', new String[]{"a","b","c"});
		letters.put('3', new String[]{"d","e","f"});
		letters.put('4', new String[]{"g","h","i"});
		letters.put('5', new String[]{"j","k","l"});
		letters.put('6', new String[]{"m","n","o"});
		letters.put('7', new String[]{"p","q","r","s"});
		letters.put('8', new String[]{"t","u","v"});
		letters.put('9', new String[]{"w","x","y","z"});

		for (String letter : letters.get(digits.charAt(0))) {
			combinations.add(letter);
		}

		for (int i=1; i<digits.length(); i++) {
			char digit = digits.charAt(i);
			List<String> newCombinations = new ArrayList<>();
			for (String newLetter : letters.get(digit)) {
				for (String oldLetter : combinations) {
					newCombinations.add(oldLetter+newLetter);
				}
			}
			combinations = newCombinations;
		}

		return combinations;        
    }
}

class Main {
	public static void main(String args[]) {
		System.out.println("Output all letter combinations");
		Solution solution = new Solution();
		List<String> testSet = new ArrayList<>();

		testSet.add("23");
		testSet.add("2345");
		testSet.add("2");
		testSet.add("");

		for (String test : testSet) {
			System.out.println("'"+test+"'");
			System.out.println("\t-> "+Arrays.toString(solution.letterCombinations(test).toArray()));
		}
	}
}
