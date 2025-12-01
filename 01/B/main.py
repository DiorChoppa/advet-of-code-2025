with open('../data', 'r', encoding='utf-8') as data:
    result = 0
    dial_counter = 50
    for line in data:
        line = line.strip()
        direction = line[0]
        step = int(line[1:])
        
        passes_through_zero = step // 100
        result += passes_through_zero
        
        remaining_step = step % 100
        if direction == 'L':
            new_position = (dial_counter - remaining_step) % 100
            if new_position > dial_counter:
                result += 1
        else:
            new_position = (dial_counter + remaining_step) % 100
            if new_position < dial_counter:
                result += 1
        
        dial_counter = new_position
    
    print(result)