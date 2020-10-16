class Solution:
    def romanToInt(self, s: str) -> int:
        return roman_to_num([table[c] for c in s])

table = {
    'M': 1000,
    'D': 500,
    'C': 100,
    'L': 50,
    'X': 10,
    'V': 5,
    'I': 1,
}

def roman_to_num(r: List[int]) -> Optional[int]:
    res = 0
    l = len(r)
    i = 0
    last_begin = float('inf')
    while i < l:
        if str(r[i]).startswith('1'):
            # begins with M/C/X/I
            if r[i] * 5 >= last_begin:
                # invalid roman, this term should smaller
                return None
            last_begin = r[i]
            if i + 1 < l and r[i + 1] == r[i]:
                # like II
                if i + 2 < l and r[i + 2] == r[i]:
                    # like III
                    res += r[i] * 3
                    i += 3
                else:
                    # like II* only
                    res += r[i] * 2
                    i += 2
            elif i + 1 < l and r[i + 1] == r[i] * 5:
                # like IV
                res += 4 * r[i]
                i += 2
            elif i + 1 < l and r[i + 1] == r[i] * 10:
                # like IX
                res += 9 * r[i]
                i += 2
            else:
                # like I only
                res += r[i]
                i += 1
        elif str(r[i]).startswith('5'):
            # starts with D/L/V
            if r[i] >= last_begin:
                # invalid roman, this term should smaller
                return None
            last_begin = r[i]
            if i + 1 < l and r[i + 1] == r[i] // 5:
                # like VI
                if i + 2 < l and r[i + 2] == r[i] // 5:
                    # like VII
                    if i + 3 < l and r[i + 3] == r[i] // 5:
                        # like VIII
                        res += r[i] // 5 * 8
                        i += 4
                    else:
                        # VII only
                        res += r[i] // 5 * 7
                        i += 3
                else:
                    # VI only
                    res += r[i] // 5 * 6
                    i += 2
            else:
                # V only
                res += r[i]
                i += 1
        else:
            # invalid
            return None
    return res
