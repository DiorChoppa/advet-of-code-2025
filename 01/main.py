with open ('data', 'r', encoding='utf-8') as data:
    result = 0
    dial_counter = 50
    for line in data:
        if 'L' in line:
          dial_counter -= int(line[1::])
        else:
          dial_counter += int(line[1::])
        dial_counter = dial_counter % 100
        print(line, ' ', dial_counter)
        if dial_counter == 0:
            result += 1
    print(result)