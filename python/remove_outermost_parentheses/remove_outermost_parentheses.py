class Solution:
    def removeOuterParentheses(self, s: str) -> str:
        result = ""
        group = ""
        opened_parenthese = 0
        for parenthese in s:
            if parenthese == "(":
                opened_parenthese = opened_parenthese + 1
            else:
                opened_parenthese = opened_parenthese - 1

            group = group + parenthese
            if opened_parenthese == 0:
                result = result + group[1:-1]
                group = ""
        return result


solution = Solution()
cases = ["(()())(())", "(()())(())(()(()))", "()()", ""]

for case in cases:
    result = solution.removeOuterParentheses(case)
    print(f"s={case}, result={result}")
