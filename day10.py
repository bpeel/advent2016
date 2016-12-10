import re
import sys

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

def get_bot(bot_num):
    if bot_num in bots:
        return bots[bot_num]

    bot = Bot(bot_num)
    bots[bot_num] = bot
    return bot

for line in sys.stdin:
    md = re.match(r'value ([0-9]+) goes to bot ([0-9]+)$',
                  line)
    if md:
        value = int(md.group(1))
        bot_num = int(md.group(2))
        get_bot(bot_num).add_chip(value)
        continue

    md = re.match(r'bot ([0-9]+) gives low to bot ([0-9]+) '
                  r'and high to bot ([0-9]+)',
                  line)
    if md:
        bot = get_bot(int(md.group(1)))
        bot.low_to = int(md.group(2))
        bot.high_to = int(md.group(3))
        continue

while True:
    full_bot = None
    
    # Look for a bot with two chips
    for (bot_num, bot) in bots.items():
        if len(bot.chips) >= 2:
            full_bot = bot
            break

    if full_bot == None:
        break

    low = min(bot.chips)
    high = max(bot.chips)
    bot.chips.clear()

    if low == 17 and high == 61:
        print("Part 1: " + str(bot.bot_num))
        break

    get_bot(bot.low_to).add_chip(low)
    get_bot(bot.high_to).add_chip(high)
