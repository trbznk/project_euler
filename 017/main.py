names = {
    0: "",
    1: "one",
    2: "two",
    3: "three",
    4: "four",
    5: "five",
    6: "six",
    7: "seven",
    8: "eight",
    9: "nine",
    10: "ten",
    11: "eleven",
    12: "twelve",
    13: "thirteen",
    14: "fourteen",
    15: "fifteen",
    16: "sixteen",
    17: "seventeen",
    18: "eighteen",
    19: "nineteen",
}

def number_to_word(number):
    if number == 0:  return ""
    if number == 1:  return "one"
    if number == 2:  return "two"
    if number == 3:  return "three"
    if number == 4:  return "four"
    if number == 5:  return "five"
    if number == 6:  return "six"
    if number == 7:  return "seven"
    if number == 8:  return "eight"
    if number == 9:  return "nine"
    if number == 10: return "ten"
    if number == 11: return "eleven"
    if number == 12: return "twelve"
    if number == 13: return "thirteen"
    if number == 14: return "fourteen"
    if number == 15: return "fifteen"
    if number == 16: return "sixteen"
    if number == 17: return "seventeen"
    if number == 18: return "eighteen"
    if number == 19: return "nineteen"
    if number < 100:
        places = {
            2: "twen",
            3: "thir",
            4: "for",
            5: "fif",
            6: "six",
            7: "seven",
            8: "eigh",
            9: "nine"
        }
        p1 = int(str(number)[-1])
        p2 = int(str(number)[-2])
        prefix = f"{places[p2]}ty" 
        postfix = number_to_word(p1)
        if len(postfix):
            return f"{prefix}-{postfix}"
        else:
            return prefix
    if number < 1000:
        p3 = int(str(number)[-3])
        prefix = f"{number_to_word(p3)} hundred"
        postfix = f"{number_to_word(int(str(number)[1:]))}" 
        if len(postfix):
            return f"{prefix} {postfix}"
        else:
            return prefix
    if number == 1000:
        return "one thousand"

    assert(False), "Unreachable"


s = 0
for i in range(1001):
    word = number_to_word(i)
    length = 0
    if i >= 100 and len(word.split()) > 2:
        length = 3
    else:
        length = 0
    length += len(word.replace("-", "").replace(" ", ""))
    s += length
    print(i, length, word)

print(s)

