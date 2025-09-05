class Solution:
    def permute(self, nums: list[int]) -> list[list[int]]:
        def helper(chain: list[int], pool: list[int], acc: list[list[int]]) -> list[list[int]]:
            if len(pool) == 0:
                acc.append(chain)
                return acc
            
            for i in range(len(pool)):
                new_chain = chain.copy()
                new_pool = pool.copy()
                num = new_pool.pop(i)
                new_chain.append(num)
                helper(new_chain, new_pool, acc)
            
            return acc
        return helper([], nums, [])
    
solution = Solution()
cases = [
    [1,2,3], [1], [1,0], [1,2,3,4]
]

for case in cases:
    ans = solution.permute(case)
    print(f"nums={case}, ans={ans}")