animals = ["bear", "tiger", "owl", "moose", "ocelot"]
print(animals[-1])

# slices - 2nd value is END (NOT LENGTH)
print(animals[0:1])
print(animals[0:2])
print(animals[2:4])

print(animals[0:-1])
print(animals[0:-2])

print(animals[:2])
print(animals[1:])

# this returns a copy
print(animals[:])

print(['a', 'b', 'c'] + [1, 2, 3])
print([1, 2, 3] * 3)

for item in animals:
    print(item)

for ix, item in enumerate(animals):
    print(ix, item)

friendliness = [3, 2, 8, 10, 11]
for a, f in zip(animals, friendliness):
    print(f"{a}: {f}")
    
print("bear" in animals)
print("shark" in animals)
print("raccoon" not in animals)
print("tiger" not in animals)

# it's actually possible to put *rest ANYWHERE in here (but only once)
bear, tiger, owl, *rest = animals
print(bear)
print(tiger)
print(owl)
print(rest)

print("owl index:", animals.index("owl"))

# this throws an exception
# print("zebra index:", animals.index("zebra"))

animals.append("lion")
print(animals)

animals.insert(0, "snow leopard")
print(animals)

del animals[animals.index("ocelot")]
print(animals)

animals.remove("owl") # only removes first instance
print(animals)

animals.pop()
print(animals)

animals.sort(reverse=True)
print(animals)

# there is also a key= argument that can be used to get the sort key for each entry
# e.g. use str.lower for case-insensitive sort, or pluck a value out of a dict

people = [
    {"name": "Jason"},
    {"name": "Alex"},
    {"name": "Bob"},
]

name_of = lambda p: p["name"]
people.sort(key=name_of)
print(people)

colours = ('red', 'green', 'yellow', 'pink', 'purple')
print(colours[1])
print(colours[1:3])
print(list(colours))
print(tuple(animals))