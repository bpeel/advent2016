import sys

if len(sys.argv) != 2:
    print("usage: day19.py <party_size>", file=sys.stderr)
    sys.exit(1)

def find_victim(guests, player):
    while True:
        player += 1
        if player >= len(guests):
            player = 0
        if guests[player] > 0:
            return player

guests = [1 for _ in range(int(sys.argv[1]))]

next_player = 0

while True:
    if guests[next_player]:
        victim = find_victim(guests, next_player)

        print("Elf {} takes {} from {}".format(next_player + 1,
                                               guests[victim],
                                               victim + 1))

        guests[next_player] += guests[victim]

        guests[victim] = 0

        if guests[next_player] >= len(guests):
            break

    next_player += 1
    if next_player >= len(guests):
        next_player = 0

print("Elf {} wins".format(next_player + 1))
