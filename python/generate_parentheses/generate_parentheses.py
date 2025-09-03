class Solution:
    def generateParenthesis(self, n: int) -> list[str]:
        mem = [[] for _ in range(n + 1)]
        mem[0] = [""]
        mem[1] = ["()"]

        for mem_k in range(2, n + 1):
            for i in range(mem_k):
                left = ["(" + p + ")" for p in mem[mem_k - i - 1]]
                right = mem[i]
                mem[mem_k] += [l + r for l in left for r in right]

        return mem[n]


def check_solution(expected, solution):
    founded = {}
    if len(expected) < len(solution):
        print("Solution has too many parentheses")
        for parenthesis in expected:
            founded[parenthesis] = 1
        for parenthesis in solution:
            if parenthesis not in founded:
                print(f"{parenthesis} is not in expected")
    elif len(expected) > len(solution):
        print("Solution has too few parentheses")
        for parenthesis in solution:
            founded[parenthesis] = 1
        for parenthesis in expected:
            if parenthesis not in founded:
                print(f"{parenthesis} is not in solution")
    else:
        print("Solution are of same lenght")


solution = Solution()
cases = [1, 2, 3]

for case in cases:
    ans = solution.generateParenthesis(case)
    print(f"n={case}, ans={ans}")


# Debug
# Case n=4
expected_4 = [
    "(((())))",
    "((()()))",
    "((())())",
    "((()))()",
    "(()(()))",
    "(()()())",
    "(()())()",
    "(())(())",
    "(())()()",
    "()((()))",
    "()(()())",
    "()(())()",
    "()()(())",
    "()()()()",
]
solution_4 = solution.generateParenthesis(4)
check_solution(expected_4, solution_4)
