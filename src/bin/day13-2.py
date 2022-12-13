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

    for i in range(min(len(left), len(right))):
        valid = compare(left[i], right[i])
        if valid != CompareResult.EQUAL:  # not exactly equal
            return valid

    if len(left) == len(right):
        return CompareResult.EQUAL
    if len(left) < len(right):
        return CompareResult.LESS
    return CompareResult.MORE


def sort(array, comp):  # bubble sort
    n = len(array)
    swapped = False
    for i in range(n-1):
        for j in range(n - i - 1):
            if comp(array[j], array[j+1]) == CompareResult.MORE:
                swapped = True
                array[j], array[j + 1] = array[j + 1], array[j]
        if not swapped:
            return


for line in file:
    if line != "\n":
        packets.append(eval(line))  # <-- this eval function is why I did today's question in python


packets.append([[2]])
packets.append([[6]])
sort(packets, compare)

index_divider_1 = packets.index([[2]]) + 1
index_divider_2 = packets.index([[6]]) + 1
print(index_divider_1, index_divider_2, index_divider_1 * index_divider_2)
