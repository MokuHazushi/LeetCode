class Solution:
    def maxCount(self, m: int, n: int, ops: list[list[int]]) -> int:
        def intersection(op1: list[int], op2: list[int]) -> list[int]:
            # op1 fully contains op2 or inversly
            if op1[0] <= op2[0] and op1[1] <= op2[1]:
                return op1
            if op2[0] <= op1[0] and op2[1] <= op1[1]:
                return op2

            # find intersection of op1 and op2
            return [min(op1[0], op2[0]), min(op1[1], op2[1])]

        biggest_intersection = [m, n]
        for op in ops:
            biggest_intersection = intersection(biggest_intersection, op)

        return biggest_intersection[0] * biggest_intersection[1]


solution = Solution()
cases = [
    {"m": 3, "n": 3, "ops": [[2, 2], [3, 3]]},
    {
        "m": 3,
        "n": 3,
        "ops": [
            [2, 2],
            [3, 3],
            [3, 3],
            [3, 3],
            [2, 2],
            [3, 3],
            [3, 3],
            [3, 3],
            [2, 2],
            [3, 3],
            [3, 3],
            [3, 3],
        ],
    },
    {"m": 3, "n": 3, "ops": [[1, 2], [2, 1]]},
]

for case in cases:
    ans = solution.maxCount(case["m"], case["n"], case["ops"])
    print(f"m={case["m"]}, n={case["n"]}, ops={case["ops"]}, ans={ans}")
