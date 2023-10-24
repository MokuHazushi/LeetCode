from typing import Optional
from typing import List
from my_collections import TreeNode

class Solution:
    def isSymmetric(self, root: Optional[TreeNode]) -> bool:
        # return self.isSymetric_rec(root.left, root.right)
        return self.isSymetric_it(root)
    
    def isSymetric_rec(self, left: Optional[TreeNode], right: Optional[TreeNode]) -> bool:
        if left == None or right == None:
            return left == right

        if left.val != right.val:
            return False
        
        return self.isSymetric_rec(left.left, right.right) and self.isSymetric_rec(left.right, right.left)
    
    def isSymetric_it(self, root: Optional[TreeNode]) -> bool:
        left, right = ([root], [root])

        while left and right:
            node_L = left.pop()
            node_R = right.pop()

            if (node_L == None or node_R == None) and (node_L != node_R):
                return False
            
            if node_L == None:
                continue

            if node_L.val != node_R.val:
                return False

            left.append(node_L.left)
            left.append(node_L.right)
            right.append(node_R.right)
            right.append(node_R.left)

        return True
    
tree = TreeNode(1, 
                TreeNode(2, TreeNode(3), TreeNode(4)),
                TreeNode(2, TreeNode(4), TreeNode(3)))
solution = Solution()
print(solution.isSymmetric(tree))