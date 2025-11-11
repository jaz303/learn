def hi(name):
    print(f"hi {name}!")

hi("Jason")
hi(name="Jason")

my_global = 100

def foo():
    print(f"my_global={my_global}")

def bar():
    my_global = 200 # creates a local
    print(f"my_global={my_global}")

def baz():
    global my_global # makes it possible to change global value
    my_global = 200
    print(f"my_global={my_global}")

foo()
bar()
print(my_global)
baz()
print(my_global)

add = lambda x, y: x + y
print(callable(add))
print(add(100, 50))

def make_adder(a):
    return lambda x: a + x

add10 = make_adder(10)
print(add10(5))