#!/usr/bin/python3

import sys
import re

FOOD_RE = re.compile(r'([a-z ]+) \(contains ([a-z, ]+)\)')

def parse_food(line):
    md = FOOD_RE.match(line)
    return (md.group(1).split(' '), md.group(2).split(', '))

foods = [parse_food(line) for line in sys.stdin]

allergens = {}

# Make a mapping from allergen name to the set of potential
# ingredients that trigger it by taking the intersection of
# ingredients from all foods that contain the allergen.
for food in foods:
    for allergen in food[1]:
        if allergen in allergens:
            allergens[allergen].intersection_update(food[0])
        else:
            allergens[allergen] = set(food[0])

# If any ingredient is now the only possibility for any allergen, we
# can remove it from the set for any other allergens. Repeat this
# until nothing changes.
while True:
    something_changed = False

    for allergen, ingredients in allergens.items():
        if len(ingredients) == 1:
            ingredient = ingredients.pop()
            for other_ingredients in allergens.values():
                if ingredient in other_ingredients:
                    other_ingredients.remove(ingredient)
                    something_changed = True
            ingredients.add(ingredient)

    if not something_changed:
        break

# Make a mapping from allergen producing ingredients to the allergen it produces
bad_ingredients = {}

for allergen, ingredients in allergens.items():
    if len(ingredients) != 1:
        raise Exception("Didn’t find single ingredient for {}".format(allergen))

    bad_ingredients[ingredients.pop()] = allergen

# Now sum up any ingredients in foods that aren’t in that list
part1 = 0
for ingredients, _ in foods:
    for ingredient in ingredients:
        if ingredient not in bad_ingredients:
            part1 += 1
        
print("Part 1: {}".format(part1))

