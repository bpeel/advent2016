import re
import sys

got_part1 = False
got_part2_count = 0
part2_result = 1

class Bin:
    def __init__(self, bin_num):
        self.bin_num = bin_num
        self.chips = []

    def add_chip(self, value):
        global part2_result, got_part2_count
        if len(self.chips) == 0 and self.bin_num < 3:
            part2_result *= value
            got_part2_count += 1
            if got_part2_count >= 3:
                print("Part 2: " + str(part2_result))
        self.chips.append(value)

class Bot:
    def __init__(self, bot_num):
        self.bot_num = bot_num
        self.low_to = None
        self.high_to = None
        self.chips = []

    def add_chip(self, value):
        if len(self.chips) >= 2:
            raise ValueError("Tried to add third chip to bot " +
                             str(self.bot_num))
        self.chips.append(value)

bots = {}
bins = {}

def get_bot(bot_num):
    if bot_num in bots:
        return bots[bot_num]

    bot = Bot(bot_num)
    bots[bot_num] = bot
    return bot

def get_bin(bin_num):
    if bin_num in bins:
        return bins[bin_num]

    bin = Bin(bin_num)
    bins[bin_num] = bin
    return bin

def get_thing(thing_name, thing_num):
    if thing_name == "output":
        return get_bin(thing_num)
    elif thing_name == "bot":
        return get_bot(thing_num)
    else:
        raise ValueError("Unknown thing: " + thing_name)

for line in sys.stdin:
    md = re.match(r'value ([0-9]+) goes to bot ([0-9]+)$',
                  line)
    if md:
        value = int(md.group(1))
        bot_num = int(md.group(2))
        get_bot(bot_num).add_chip(value)
        continue

    md = re.match(r'bot ([0-9]+) gives low to ([a-z]+) ([0-9]+) '
                  r'and high to ([a-z]+) ([0-9]+)',
                  line)
    if md:
        bot = get_bot(int(md.group(1)))
        bot.low_to = get_thing(md.group(2), int(md.group(3)))
        bot.high_to = get_thing(md.group(4), int(md.group(5)))
        continue

    raise ValueError("Invalid line: " + line.rstrip())

while not got_part1 or got_part2_count < 3:
    full_bot = None
    
    # Look for a bot with two chips
    for (bot_num, bot) in bots.items():
        if len(bot.chips) >= 2:
            full_bot = bot
            break

    if full_bot == None:
        print("No bot with two chips found")
        break

    low = min(bot.chips)
    high = max(bot.chips)
    bot.chips.clear()

    if low == 17 and high == 61 and not got_part1:
        print("Part 1: " + str(bot.bot_num))
        got_part1 = True

    bot.low_to.add_chip(low)
    bot.high_to.add_chip(high)
