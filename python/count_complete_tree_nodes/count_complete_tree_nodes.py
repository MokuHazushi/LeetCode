# Definition for a binary tree node.
import Optional
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
class Solution:
    def countNodes(self, root: Optional[TreeNode]) -> int:
        def is_complete(tree) -> bool:
            left = tree.left
            right = tree.right
            depth = 1
            while left != None and right != None:
                left = left.left
                right = right.right
                depth +=1

            if left == None and right == None:
                return pow(2, depth) - 1
            return 0

        def traversal(node) -> int:
            if node == None:
                return 0
            count = is_complete(node)
            if count != 0:
                return count

            return 1 + traversal(node.left) + traversal(node.right)

        return traversal(root)
            
