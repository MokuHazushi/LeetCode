// https://leetcode.com/problems/network-delay-time/

package network_delay_time;

import java.util.List;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Hashtable;
import java.util.PriorityQueue;
import java.util.Map;
import java.util.Queue;

class Neighbor implements Comparable<Neighbor> {
	int node, cost;

	public Neighbor(int node, int cost) {
		this.node = node;
		this.cost = cost;
	}

	@Override
	public int compareTo(Neighbor o) {
		return new Integer(cost).compareTo(o.cost);
	}
}

class Solution {
    public int networkDelayTime(int[][] times, int n, int k) {
		Map<Integer, List<Neighbor>> adjencyMap = new Hashtable<>();
		int[] costs = new int[n];
		Queue<Neighbor> search = new PriorityQueue<>();

		for (int[] time : times) {
			adjencyMap.putIfAbsent(time[0], new ArrayList<>());
			adjencyMap.get(time[0]).add(new Neighbor(time[1], time[2]));
		}

		Arrays.fill(costs, Integer.MAX_VALUE);
		costs[k-1] = 0;
		search.add(new Neighbor(k, costs[k-1]));

		while (search.size() > 0) {
			int node = search.poll().node;

			if (!adjencyMap.containsKey(node))
				continue;
			
			for (Neighbor neighbor : adjencyMap.get(node)) {
				if (costs[neighbor.node-1] > costs[node-1] + neighbor.cost) {
					costs[neighbor.node-1] = costs[node-1] + neighbor.cost;
					search.add(new Neighbor(neighbor.node, costs[neighbor.node-1]));
				}
			}
		}
		
		int maxCost = -1;
		for (int cost : costs)
			maxCost = Math.max(maxCost, cost);
		
		if (maxCost == Integer.MAX_VALUE)
			return -1;
		return maxCost;
    }
}

class Test {
	int[][] times;
	int n, k;

	public Test(int[][] times, int n, int k) {
		this.times = times;
		this.n = n;
		this.k = k;
	}
}

class Main {
	public static void main(String args[]) {
		System.out.println("Compute network delay time");
		Solution solution = new Solution();
		List<Test> testSet = new ArrayList<>();

		int[][] times1 = new int[][] {
			{2,1,1},
			{2,3,1},
			{3,4,1}
		};
		testSet.add(new Test(times1, 4, 2));

		int[][] times2 = new int[][] {
			{1,2,1},
			{2,3,2},
			{1,3,1}
		};
		testSet.add(new Test(times2, 3, 2));

		int[][] times3 = new int[][] {
			{1,2,1},
			{1,3,1},
			{2,4,10},
			{3,4,1}
		};
		testSet.add(new Test(times3, 4, 1));

		int[][] times4 = new int[][] {
			{1,2,2},
			{1,4,1},
			{2,3,1},
			{3,5,1},
			{4,5,4},
			{5,6,1}
		};
		testSet.add(new Test(times4, 6, 1));

		for (Test test : testSet) {
			String times = "";
			for (int[] time : test.times)
				times += Arrays.toString((time)) + ", ";
			System.out.println("Times="+times + " n=" + test.n + " k=" + test.k);
			System.out.println("\t-> " + solution.networkDelayTime(test.times, test.n, test.k));
		}
	}
}
