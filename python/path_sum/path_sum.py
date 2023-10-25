from typing import Optional
from typing import List

from my_collections import TreeNode

class Solution:
    def hasPathSum(self, root: Optional[TreeNode], targetSum: int) -> bool:
        if not root:
            return False
        # return self.hasPathSum_rec(root, targetSum, 0)
        return self.hasPathSum_it(root, targetSum)
    
    def hasPathSum_rec(self, node: Optional[TreeNode], targetSum: int, acc: int) -> bool:
        if not node:
            return targetSum == acc
        return self.hasPathSum_rec(node.left, targetSum, acc+node.val) \
            or self.hasPathSum_rec(node.right, targetSum, acc+node.val)
    
    def hasPathSum_it(self, root: Optional[TreeNode], targetSum: int) -> bool:
        stack = [(root, root.val)]

        while stack:
            node, acc = stack.pop()

            if (not node.left) and (not node.right):
                if acc == targetSum:
                    return True
                else:
                    continue
            
            if node.right:
                stack.append((node.right, acc+node.right.val))
            if node.left:
                stack.append((node.left, acc+node.left.val))
        return False
                
                
            

# Main
solution = Solution()
tree1 = TreeNode(5, 
                 TreeNode(4, 
                          TreeNode(11, TreeNode(7), TreeNode(2)),
                          None),
                TreeNode(8,
                         TreeNode(13), TreeNode(4, None, TreeNode(1))))

print("Tree1 -> {}".format(solution.hasPathSum(tree1, 22)))