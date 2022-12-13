import enum

file = open("../input/day13.txt", "r")
packets = []


class CompareResult(enum.Enum):
    LESS = 1
    EQUAL = 2
    MORE = 3


def compare(left, right) -> CompareResult:  # 0 -> exactly equal, >0 -> not in right order, <0 in right order
    if type(left) == int:
        if type(right) == int:
            if left == right:
                return CompareResult.EQUAL
            if left < right:
                return CompareResult.LESS
            return CompareResult.MORE
        else:
            return compare([left], right)
    if type(right) == int:
        return compare(left, [right])

    for index in range(min(len(left), len(right))):
        valid = compare(left[index], right[index])
        if valid != CompareResult.EQUAL:  # not exactly equal
            return valid

    if len(left) == len(right):
        return CompareResult.EQUAL
    if len(left) < len(right):
        return CompareResult.LESS
    return CompareResult.MORE


pair = []
for line in file:
    if line == "\n":
        packets.append(pair)
        pair = []
    else:
        pair.append(eval(line))  # <-- this eval function is why I did today's question in python

# print(packets)

inorder = []
for packet in packets:
    inorder.append(compare(packet[0], packet[1]))
    # print(inorder[-1])

# print(inorder)

result = 0
for i, v in enumerate(inorder):
    if v == CompareResult.LESS:
        result += (i + 1)

print(result)
