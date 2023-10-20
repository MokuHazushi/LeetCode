from typing import Optional
from typing import List
from my_collections import TreeNode

class Solution:
    # Pre-order traversal: root -> left -> right
    def preorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        #return self.preorder_rec(root)
        return self.preorder_it(root)
    
    def preorder_rec(self, root: Optional[TreeNode]) -> List[int]:
        ans = []

        def rec(node):
            if node == None:
                return
            ans.append(node.val)
            rec(node.left)
            rec(node.right)
        
        rec(root)
        return ans
    
    def preorder_it(self, root: Optional[TreeNode]) -> List[int]:
        ans, stack = [], [root]
        
        while len(stack) != 0:
            node = stack.pop()
            if node == None:
                continue
            ans.append(node.val)
            stack.append(node.right)
            stack.append(node.left)
        
        return ans
    
    # In-order traversal: left -> root -> right
    def inorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        return self.inorder_it(root)
    
    def inorder_rec(self, root: Optional[TreeNode]) -> List[int]:
        ans = []

        def rec(node):
            if node == None:
                return
            
            rec(node.left)
            ans.append(node.val)
            rec(node.right)
        
        rec(root)
        return ans
    
    def inorder_it(self, root: Optional[TreeNode]) -> List[int]:
        ans, stack = [], []
        node = root

        while len(stack) != 0 or node != None:
            while node != None:
                stack.append(node)
                node = node.left
            
            node = stack.pop()
            ans.append(node.val)
            node = node.right
        
        return ans
    
    # Post-order traversal: left -> right -> root
    def postorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        return self.postorder_it(root)
    
    def postorder_rec(self, root: Optional[TreeNode]) -> List[int]:
        ans = []

        def rec(node):
            if node == None:
                return
            rec(node.left)
            rec(node.right)
            ans.append(node.val)
        
        rec(root)
        return ans

    def postorder_it(self, root: Optional[TreeNode]) -> List[int]:
        stack, ans = [], []
        node = root

        while len(stack) != 0 or node != None:
            while node != None:
                if node.right != None:
                    stack.append(node.right)
                stack.append(node)
                node = node.left
            
            node = stack.pop()
            if len(stack) != 0 and node.right == stack[len(stack)-1]:
                right = stack.pop()
                stack.append(node)
                node = right
            else:
                ans.append(node.val)
                node = None
        
        return ans


    

    
tree = TreeNode(1, TreeNode(2), TreeNode(3))
solution = Solution()

print("Pre-order:")
print(solution.preorderTraversal(tree))

print("In-order:")
print(solution.inorderTraversal(tree))

print("Post-order:")
print(solution.postorderTraversal(tree))