class Solution:
    def smallestRangeI(self, nums: list[int], k: int) -> int:
        min = 10000
        max = 0

        for num in nums:
            if num < min:
                min = num
            if num > max:
                max = num

        if max - min <= 2 * k:
            return 0
        return max - min - 2 * k
    
solution = Solution()
cases = [
    {
        "nums": [1], 
        "k": 0
    },
        {
        "nums": [0, 10], 
        "k": 2
    },
    {
        "nums": [1, 3, 6], 
        "k": 3
    }
]

for case in cases:
    ans = solution.smallestRangeI(case["nums"], case["k"])
    print(f"nums={case["nums"]}, k={case["k"]}, ans={ans}")