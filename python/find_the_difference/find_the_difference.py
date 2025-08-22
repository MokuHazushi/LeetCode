class Solution:
    def findTheDifference(self, s: str, t: str) -> str:
        count = {}
        for letter in s:
            if letter not in count:
                count[letter] = 1
            else:
                count[letter] = count[letter] + 1

        for letter in t:
            if letter not in count:
                return letter
            count[letter] = count[letter] - 1

            if count[letter] < 0:
                return letter
            
solution = Solution()
cases = [
    ("abcd", "abcde"),
    ("", "y"),
    ("aaa", "aaaa")
]

for case in cases:
    result = solution.findTheDifference(case[0], case[1])
    print(f"s={case[0]}, t={case[1]}, answer={result}")