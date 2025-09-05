class Solution:
    def sumZero(self, n: int) -> list[int]:
        ans = []

        if n % 2 != 0:
            ans.append(0)
            n = n - 1
        if n > 0 and n % 2 == 0:
            k = 1
            for _ in range((int)(n/2)):
                ans.append(k)
                ans.append(-k)
                k = k + 1

        return ans
    
solution = Solution()
cases = [
    1, 2, 3, 4, 5, 6, 100
]

for case in cases:
    ans = solution.sumZero(case)
    sum = 0
    for k in ans:
        sum = sum + k
    
    print(f"n={case}, ans={ans}, sum is {sum}")
