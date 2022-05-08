// https://leetcode.com/problems/delete-and-earn/

import java.util.Arrays;

import java.util.Hashtable;
import java.util.List;
import java.util.ArrayList;
import java.lang.Math;

class Solution {
    public int deleteAndEarn(int[] nums) {
        Hashtable<Integer, Integer> occurences = new Hashtable<>();
        List<Integer> sortedKeys;
        int[] earn = {0, 0};
        
        for (int n : nums) {
            if (occurences.containsKey(n))
                occurences.put(n, occurences.get(n)+1);
            else
                occurences.put(n, 1);
        }

        sortedKeys = new ArrayList<>(occurences.keySet());
        sortedKeys.sort(null);
        earn[1] = sortedKeys.get(0) * occurences.get(sortedKeys.get(0));

        for (int i=1; i<sortedKeys.size(); i++) {
            int key = sortedKeys.get(i);
            int newValue = key * occurences.get(key);

            if (sortedKeys.get(i-1) == key-1)
                newValue += earn[0];
            else
                newValue += Math.max(earn[1], earn[0]);
            
            if (newValue > 0) {
                earn[0] = Math.max(earn[1], earn[0]);
                earn[1] = newValue;
            }
        }

        return Math.max(earn[0], earn[1]); 
    }
}

class Main {
    public static void main(String args[]) {
        Solution solution = new Solution();
        List<int[]> testSet = new ArrayList<>();

        testSet.add(new int[]{1});
        testSet.add(new int[]{1,2});
        testSet.add(new int[]{3,1});
        testSet.add(new int[]{3,4,2});
        testSet.add(new int[]{3,3,3,2,2,4});
        testSet.add(new int[]{1,1,1,2,4,5,5,5,6});

        for (int[] test : testSet) {
            System.out.println("nums="+Arrays.toString(test)+" -> "+solution.deleteAndEarn(test));
        }
    }
}