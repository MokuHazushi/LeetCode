class Solution:
    def commonChars(self, words: list[str]) -> list[str]:
        ans = {}
        for letter in words[0]:
            if letter not in ans:
                ans[letter] = 1
            else:
                ans[letter] = ans[letter] + 1

        for word in words[1:]:
            pool = {}
            for letter in word:
                if letter in ans and ans[letter] > 0:
                    if letter not in pool:
                        pool[letter] = 1
                    else:
                        pool[letter] = pool[letter] + 1

                    ans[letter] = ans[letter] - 1

            ans = pool
        letters = []
        for key, value in ans.items():
            for _ in range(value):
                letters.append(key)
        return letters


# Testing
solution = Solution()
cases = [
    ["bella", "label", "roller"],
    ["cool", "lock", "cook"],
    ["abc"],
    ["abc", "efg"],
]

for case in cases:
    input = case
    result = solution.commonChars(case)
    print(f"input={input}, ans={result}")
