s1 = {1, 2, 2, 3, 1, 3, 1, 2, 2} # this only works when non-empty, {} is a dictionary
print(s1)

s2 = set((3, 1, 2, 2, 2, 2, 3, 1, 3))
print(s2)

s1.add(4)
print(s1)

s2.update([10, 11, 12, 13, 14])
print(s2)


s2.remove(1) # this will raise if the item doesn't exist
print(s2)

for i in range(10, 20):
    s2.discard(i) # this won't raise errors if value isn't in set

print(s2)

# also: a | b
foo = set([1, 2, 3]).union(set([3, 4, 5]))
print(foo)

# also: a & b
bar = set([1, 2, 3]).intersection(set([3, 4, 5]))
print(bar)

# also: a - b
baz = set([1, 2, 3]).difference(set([3, 4, 5]))
print(baz)

# also: a ^ b
bleem = set([1, 2, 3]).symmetric_difference(set([3, 4, 5]))
print(bleem)