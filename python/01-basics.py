print(16 / 3)
print(15 // 4)

x = 0
x += 1
x -= 2
x *= 100
print("x:", x)

lst = ['a', 'b', 'c']
lst *= 3
print("lst:", lst)

# assignment expression (walrus operator)
print(y := 100)
print(y)

# complex numbers
print(2+3j)
print(2 + 3j)
print(complex(2, 3))

# conact strings
name = 'Streetcat' 'Bob'
print(name)

triple_name = name * 3
print(triple_name)

def my_func():
    """
    This is the function docstring
    """
    pass

# Printing
words = ['these', 'are', 'some', 'words']
print(*words, sep='_', end='!') # end changes the newline after output
print("")

# Input
# name = input()
name = 'Jason'
print("Hi {}".format(name))
print(f"Hi {name}")

print("len(name):", len(name))
print("len(words):", len(words))

# empty list/dict/string are falsey
if [] or {} or "":
    print("shouldn't see this!!!")

# type conversions
a = str(29)
b = str(1.1)
c = int("100")
d = int(3.14)
e = float("150")
f = float("200.19")
g = float(50)
# h = int("225.67") - error because "255.67" is not a valid int
print(a, b, c, d, e, f, g)