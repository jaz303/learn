names = ['Jason', 'Bob', 'Charlie', 'Alfred', 'Chris']

copy = [n for n in names]
print(copy)

c_names = [n for n in names if n.startswith('C')]
print(c_names)

# cartesian product?
vals = [(a, b) for a in range(1, 5) for b in range(1, 5)]
print(vals)

nums = [1, 2, 3, 4, 5, 6]
new_list = [(n*2 if n%2==0 else n) for n in nums]
print(new_list)

# set from list
nums = [1, 2, 1, 3, 1, 2, 2, 2, 1, 1, 1, 3, 1, 2]
unique_nums = {n for n in nums}
print(unique_nums)

# dict from list of tuples
people = [
    ("Jason", 44, "noodles"),
    ("Bob", 50, "soup"),
    ("Alice", 38, "chicken teriyaki")
]

favourite_foods = {name:food for (name, age, food) in people}
print(favourite_foods)

c = {'name': 'Jason', 'age': 44}
v = ["{}:{}".format(k.upper(), v) for k,v in c.items()]
print(v)