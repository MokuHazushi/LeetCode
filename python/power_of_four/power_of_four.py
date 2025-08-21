class Solution:
    def isPowerOfFour(self, n: int) -> bool:
        import math

        if n == 1:
            return True
        if n < 3:
            return False

        return (n & (n - 1) == 0) and math.floor(
            math.log2(n - 1)
        ) % 2 == 1  # This work but you could also use n % 3 == 1


solution = Solution()
cases = [1, 4, 16, 8, -1, 15, 32, 64]
for case in cases:
    result = solution.isPowerOfFour(case)
    print(f"n={case}, result={result}")
