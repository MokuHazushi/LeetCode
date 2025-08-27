class Solution:
    def arrangeCoins(self, n: int) -> int:
        import math

        def successive_sum(k):
            return (k * (k + 1)) / 2

        # Binary search solution
        left = 1
        right = pow(2, 16)

        while left + 1 < right:
            middle = math.floor((right - left) / 2) + left
            coins = successive_sum(middle)

            if coins == n:
                return middle
            if coins < n:
                left = middle
            if coins > n:
                right = middle

        return left

        # Loop based solution
        # row = 1
        # n = n - row

        # while n > 0:
        #     row += 1
        #     n = n - row

        # if n == 0:
        #     return row
        # return row - 1


solution = Solution()
cases = [1, 3, 5, 8, pow(2, 31) - 1]

for case in cases:
    ans = solution.arrangeCoins(case)
    print(f"n={case}, ans={ans}")
