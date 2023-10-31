package regex_matching;

import java.util.ArrayList;
import java.util.List;

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
	List<Node> neighbors;
	
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
    	
    	return false;        
    }
}

class Main {
    public static void main(String args[]) {
        System.out.println("Does string 's' matches pattern 'p'?");
        String[] testCases = new String[] {
        		"aa", "a",
        		"aa", "a*",
        		"ab", ".*"
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
