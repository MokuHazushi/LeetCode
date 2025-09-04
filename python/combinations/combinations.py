class Solution:
    def combine(self, n: int, k: int) -> list[list[int]]:
        last_combination = [i for i in range(1, k + 1)]
        ans = []
        ans.append(last_combination.copy())
        i = 0
        while i < k:
            if last_combination[k - i - 1] < n - i:
                last_combination[k - i - 1] += 1
                for j in range(k - i, k):
                    last_combination[j] = last_combination[j - 1] + 1
                ans.append(last_combination.copy())

                if k - i < k:
                    i = 0
                continue
            i += 1
        return ans


solution = Solution()
cases = [[4, 2], [1, 1], [5, 3]]

for case in cases:
    ans = solution.combine(case[0], case[1])
    print(f"n={case[0]}, k={case[1]}, ans={ans}")
