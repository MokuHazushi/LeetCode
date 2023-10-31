package regex_matching;

import java.util.ArrayList;
import java.util.List;
import java.util.Stack;

class Token {
	enum Type { 
		LETTER, DOT, STAR;
	}
	
	final Type type;
	char letter;
	
	public Token(Type type, char letter) {
		this.type = type;
		this.letter = letter;
	}
	
	public static Token toToken(char c) {
		if (c == '.') {
			return new Token(Type.DOT, c);
		}
		if (c == '*') {
			return new Token(Type.STAR, c);
		}
		return new Token(Type.LETTER, c);
	}
}

class Node {
	final Token token;
	List<Node> neighbors; // Use 2 neighbors variable instead of a list
	
	public Node(Token token, Node... neighbors) {
		this.token = token;
		this.neighbors = List.of(neighbors);
	}
	
	public Node insertNode(Node newNode) {
		if (newNode.token.type == Token.Type.STAR)
			return insertStarNode(newNode);
		
		newNode.neighbors = List.copyOf(this.neighbors);
		this.neighbors = List.of(newNode);
		return newNode;
	}
	
	Node insertStarNode(Node starNode) {
		starNode.neighbors = new ArrayList<>();
		starNode.neighbors.add(starNode);
		starNode.neighbors.addAll(this.neighbors);
		
		List<Node> newNeighbors = new ArrayList<>();
		newNeighbors.addAll(this.neighbors);
		newNeighbors.add(starNode);
		this.neighbors = newNeighbors;
		
		return starNode;
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
    	List<Token> tokens = new ArrayList<>();
    	Node graph = new Node(null, new Node(null));
    	Node curNode = graph;
    	
    	// Lexer
    	for (char c : p.toCharArray())
    		tokens.add(Token.toToken(c));
    	
    	// Parser
    	for (Token token : tokens) {
    		Node newNode = new Node(token);
    		curNode = curNode.insertNode(newNode);
    	}
    	
    	// Search
    	Stack<SearchElement> stack = new Stack<>();
    	stack.push(new SearchElement(graph.neighbors.get(0), 0));
    	
    	while (!stack.empty()) {
    		SearchElement search = stack.pop();
    		int index = search.index;
    		Node node = search.node;
    		
    		if (node.token == null && index == s.length())
    			return true;
    		
    		if (node.token == null || index == s.length())
    			continue;
    		
    		switch(node.token.type) {
    		case LETTER:
    			if (node.token.letter != s.charAt(index))
    				continue;
    			for (Node neighbor : node.neighbors)
    				stack.push(new SearchElement(neighbor, index+1));
    			break;
    		case DOT:
    		case STAR:
    			for (Node neighbor : node.neighbors)
    				stack.push(new SearchElement(neighbor, index+1));
    			break;
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
