import sys

# Comparison ops: ==, !=, <, <=, >, >=
# Logical ops: and, or, not

name = "Anatoly"
if name == "Jason":
    print("you are jason")
elif name == "Anatoly":
    print("i mop here!")
else:
    print("you are not Jason")

# ternary 
age = 20
old = True if age > 20 else False
print("old", old)

# match
code = 500
match code:
    case 200 | 201:
        print("Success")
    case 404:
        print("Not Found")
    case _:
        print("something else entirely")

vals = [1, 2, 3, 4, 5, 6]
match vals:
    case [a]:
        print(f"a={a}")
    case [a,b]:
        print(f"a={a} b={b}")
    case [a,b,*rest]:
        print(f"a={a} b={b}, c={rest}")

foo = "foo"
match foo:
    case int():
        print("foo is an int")
    case str():
        print("foo is a string")
    case _:
        print("foo is something else")

count = 0
while count < 5:
    print("hi!")
    count += 1
    
# break and continue are things too

for foo in [1, 2, 3]:
    print(foo)

# else statement only runs if the for loop runs to completion (i.e. no break) 
for i in [1,2,4,5]:
    if i == 3:
        break
else:
    print("break was not called!")

my_dict = {"a": 1, "b": 2, "c": 3}

# iterate by keys
for k in my_dict:
    print(k)

for v in my_dict.values():
    print(v)

print(type(my_dict.values()))

for k, v in my_dict.items():
    print(f"{k} => {v}")

for i, (k, v) in enumerate(my_dict.items()):
    print(f"{i}: {k} => {v}")
    
print(range(10))
print(type(range(10)))

for i in range(10):
    print(i)

for i in range(100, 0, -25):
    print(i)

sys.exit()