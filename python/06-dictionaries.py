cat = {
    'name': 'dora',
    'size': 'chonk',
    'color': 'calico',
    'disposition': 'needy'
}

cat['age'] = 15

print(cat.values())
print(cat.keys())
print(cat.items())

print(cat.get("favourite_food"))
print(cat.get("favourite_food", "tuna"))
print("foo" in cat)

foo = {'bar': 'bleem', 'baz': 123, 'quux': 'mordor'}

foo.setdefault('bar', 'baz')

# setdefault is shorthand for:
if 'bar' not in foo:
    foo['bar'] = 'baz'

print(foo)

foo.pop('bar')
print(foo)

# removes the last item from the directory and returns it
print("popitem():", foo.popitem())
print(foo)

del foo['baz']
print(foo)


my_dict = {1: 2, 3: 4, 5: 6}
my_dict.clear()
print(my_dict)

# checking
my_dict = {'a': 1, 'b': 2, 'c': 3}
print('a' in my_dict.keys())
print('a' in my_dict)
print(2 in my_dict.values())

# splat merging
a = {1: 2}
b = {3: 4}
c = {**a, **b}
print(c)