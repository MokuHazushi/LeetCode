from typing import Optional
from typing import List
from my_collections import TreeNode

class Solution:
    def isSymmetric(self, root: Optional[TreeNode]) -> bool:
        return self.isSymetric_rec(root.left, root.right) 
    
    def isSymetric_rec(self, left, right) -> bool:
        if left == None or right == None:
            return left == right

        if left.val != right.val:
            return False
        
        return self.isSymetric_rec(left.left, right.right) and self.isSymetric_rec(left.right, right.left)
    
tree = TreeNode(1, 
                TreeNode(2, TreeNode(3), TreeNode(4)),
                TreeNode(2, TreeNode(4), TreeNode(3)))
solution = Solution()
print(solution.isSymmetric(tree))