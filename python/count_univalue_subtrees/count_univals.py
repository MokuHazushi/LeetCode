from typing import Optional
from my_collections import TreeNode

class Solution:
    def countUnivalSubtrees(self, root: Optional[TreeNode]) -> int:
        if not root:
            return 0
        
        def helper(node):
            if (not node.left) and (not node.right):
                return (1, True)
            
            ans = 0
            uniVal = True
            if node.left:
                ansLeft, uniLeft = helper(node.left)
                ans += ansLeft
                uniVal = uniVal and (node.left.val == node.val) and uniLeft
                
            if node.right:
                ansRight, uniRight = helper(node.right)
                ans += ansRight
                uniVal = uniVal and (node.right.val == node.val) and uniRight
            
            if uniVal:
                ans += 1
            return (ans, uniVal)

        return helper(root)[0]

            
        

    
# Main
solution = Solution()
tree1 = TreeNode(5,
                 TreeNode(1, TreeNode(5), TreeNode(5)),
                 TreeNode(5, None, TreeNode(5)))
print("Tree1 -> {}".format(solution.countUnivalSubtrees(tree1)))