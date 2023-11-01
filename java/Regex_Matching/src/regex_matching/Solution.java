package regex_matching;

import java.util.ArrayList;
import java.util.List;
import java.util.Stack;

class Token {
	final char symbol;
	
	public Token(char symbol) {
		this.symbol = symbol;
	}
	
	public boolean accept(char letter) {
		if (symbol == '.')
			return true;
		return symbol == letter;
	}
}

class NodeBox {
	Node node;
}

class Node {
	final Token token;
	NodeBox next, starNext, skipNext;
	
	public Node(Token token) {
		this.token = token;
		this.next = new NodeBox();
		this.starNext = new NodeBox();
		this.skipNext = new NodeBox();
	}
	
	public Node(Token token, Node neighbor) {
		this(token);
		this.next.node = neighbor;
	}
	
	public List<Node> getNeighbors() {
		List<Node> neighbors = new ArrayList<>();
		neighbors.add(next.node);
		if (starNext.node != null)
			neighbors.add(starNext.node);
		if (skipNext.node != null)
			neighbors.add(skipNext.node);
		return neighbors;
	}
	
	public Node insertNode(Token next) {
		Node newNode = new Node(next);
		newNode.next.node = this.next.node;
		this.next.node = newNode;
		return newNode;
	}
	
	public Node insertStarNode(Token next) {
		Node newNode = new Node(next);
		newNode.next.node = this.next.node;
		newNode.starNext.node = newNode;
		this.skipNext = newNode.next;
		this.next.node = newNode;
		return newNode;
	}
}

class SearchElement {
	final Node node;
	final int index;
	
	public SearchElement(Node node, int index) {
		super();
		this.node = node;
		this.index = index;
	}
}

/** LEETCODE CLASS **/
class Solution {
    public boolean isMatch(String s, String p) {
    	Node graph = new Node(null, new Node(null));
    	Node curNode = graph;
    	
    	// Parser
    	for (int i=0; i<p.length(); i++) {
    		if (p.charAt(i) == '*')
    			continue;
    		
    		Token next = new Token(p.charAt(i));
    		
    		if (i < p.length()-1 && p.charAt(i+1) == '*')
    			curNode = curNode.insertStarNode(next);
    		else
    			curNode = curNode.insertNode(next);
    	}
    	
    	// Search
    	Stack<SearchElement> stack = new Stack<>();
    	for (Node node : graph.getNeighbors())
    		stack.push(new SearchElement(node, 0));
    	
    	while (!stack.empty()) {
    		SearchElement search = stack.pop();
    		int index = search.index;
    		Node node = search.node;
    		
    		if (node.token == null && index == s.length())
    			return true;
    		
    		if (node.token == null || index == s.length())
    			continue;
    		
    		if (node.token.accept(s.charAt(index))) {
    			for (Node neighbor : node.getNeighbors())
    				stack.push(new SearchElement(neighbor, index+1));
    		}
    	}
    	
    	return false;
    }
}

class Main {
    public static void main(String args[]) {
        System.out.println("Does string 's' matches pattern 'p'?");
        String[] testCases = new String[] {
        		/*
        		"aa", "a",
        		"aa", "a*",
        		"ab", ".*",
        		*/
        		"aab", "c*a*b"
        };
 		Solution solution = new Solution();
 		int i = 0;
 		
 		while (i < testCases.length) {
 			String s = testCases[i];
 			String p = testCases[i+1];
 			System.out.println("s='"+s+"', p='"+p+"' -> "+solution.isMatch(s, p));
 			i += 2;
 		}
 	}
}
