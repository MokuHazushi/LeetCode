from typing import List

class Solution:
    def rob(self, nums: List[int]) -> int:
        if len(nums) == 1:
            return nums[0]
        if len(nums) == 2:
            return max(nums[0], nums[1])
        
        memory = [0 for x in range(len(nums))]
        memory[0] = nums[0]
        memory[1] = nums[1]
        i = 2
        while i < len(nums):
            memory[i] = max(nums[i]+memory[i-2], memory[i-1])
            i += 1
        
        return max(memory[-1], memory[-2])
        

    
solution = Solution()
nums = [1,2,3,1]
print(solution.rob(nums))