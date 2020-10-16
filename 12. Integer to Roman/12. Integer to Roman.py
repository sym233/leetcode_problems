class Solution:
    def intToRoman(self, num: int) -> str:
        return ''.join([romans[n] for n in num_to_r(num)])
        
        
        
romans = {
    1: 'I', 
    5: 'V', 
    10: 'X',
    50: 'L',
    100: 'C',
    500: 'D',
    1000: 'M',
}

mapping = [
    [], # 0
    [1], # 1
    [1, 1], # 2
    [1, 1, 1], # 3
    [1, 5], # 4
    [5], # 5
    [5, 1], # 6
    [5, 1, 1], # 7
    [5, 1, 1, 1], # 8
    [1, 10], # 9
]

def num_to_r(num: int) -> List[int]:
    r = [mapping[int(c)] for c in str(num)]
    l = len(r)
    for i in range(l):
        r[i] = [n * 10 ** (l - i - 1) for n in r[i]]
    return [n for ri in r for n in ri]
