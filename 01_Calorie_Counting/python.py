



def elves(input):
    input = input.split('\n')
    calorie_count = 0
    elf_list = []
    elf_count = 0
    result = 0

    for i in input:
        if i == '':
            elf_count += 1
            elf_list.append(calorie_count)
            calorie_count = 0
            continue
        calorie_count += int(i)
    for j in elf_list:
        if j > result:
            result = j
    return result

print(elves(input))
