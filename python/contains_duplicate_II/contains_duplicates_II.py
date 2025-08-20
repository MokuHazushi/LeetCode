class Solution:
    def containsNearbyDuplicate(self, nums: list[int], k: int) -> bool:
        last_occurence = {}
        if k == 0:
            return False
        for i in range(len(nums)):
            num = nums[i]
            last = last_occurence.get(num)
            if last == None or i - last > k:
                last_occurence[num] = i
                continue

            return True
        return False


# Main
solution = Solution()

cases = []
# Leetcode cases
cases.append(([1, 2, 3, 1], 3))
cases.append(([1, 0, 1, 1], 1))
cases.append(([1, 2, 3, 1, 2, 3], 2))

for case in cases:
    nums = case[0]
    k = case[1]
    answer = solution.containsNearbyDuplicate(nums, k)
    print(f"nums={nums}, k={k}, answer={answer}")
