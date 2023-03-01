def get_inputs(filename: str):
    with open(filename, "r") as file:
        lines = file.read().splitlines()

    first = lines[0].split(" ")
    N = int(first[0])
    M = int(first[1])

    p = lines[1].split(" ")

    prices = []

    for v in p:
        prices.append(int(v))

    n = lines[2].split(" ")

    numbers = []

    for v in n:
        numbers.append(int(v))

    return (N, M, prices, numbers)


def find_free(arr: list) -> int:
    for i in range(len(arr)):
        if arr[i] == -1:
            return i


def find_car(arr: list, number: int) -> int:
    for i in range(len(arr)):
        if arr[i] == number:
            return i


for i in range(1, 5):
    N, M, prices, numbers = get_inputs("input3/input."+str(i))

    spaces = [-1 for x in range(N)]
    amount = 0
    count = 0
    waiting = []

    while len(numbers) > 0:
        if count == N:
            while numbers[0] > 0:
                waiting.append(numbers.pop(0))
            number = numbers.pop(0)
        else:
            if len(waiting) > 0:
                number = waiting.pop(0)
            else:
                number = numbers.pop(0)
        if number > 0:
            space = find_free(spaces)
            spaces[space] = number
            count += 1
        else:
            space = find_car(spaces, -number)
            spaces[space] = -1
            amount += prices[space]
            count -= 1

    print(amount)
