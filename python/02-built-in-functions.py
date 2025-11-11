# abs()
print(abs(-1))
print(abs(1.23))
print(abs(-1.23))

# aiter()
# Returns an asynchronous iterator for an asynchronous iterable
# Let's leave this for now, seems a bit advanced...

# all()
# Returns True if all elements in iterable are true
# WHAT IS AN "ITERABLE" IN PYTHON?
print("all()")
print(all([True, True, True]))
print(all([True, False, True]))
print()

# any()
print("any()")
print(any([False, False, False]))
print(any([False, True, False]))
print()

# ascii()
# Similar to repr() but it makes sure output is ASCII-compliant by escaping stuff
# Seems to generate strings that are valid Python? Dunno, lets wait until we get to repr()
print("ascii()")
print(ascii('A'))
print(ascii(["foo", "bar", "baz"]))
print()

# bin()
# Print as binary string
print("bin()")
print(bin(1204))
print()

# breakpoint()
# Allows you to enter the debugger

# bytearray()
# Returns a new array of bytes
# bytearray is a class representing a MUTABLE sequence of bytes
# mutable version of "bytes"
print("bytearray()")
ba1 = bytearray("hello world", "utf-8")
print(ba1)
ba2 = bytearray([72, 101, 108, 108, 111])
print(ba2)
ba2[0] = 104
ba2.append(33)
print(ba2)
print()

# bytes()
# bytes type is an immutable sequence of bytes
print("bytes()")
data = "foobar"
data_bytes = bytes(data, "utf-8")
print(data_bytes)
print(data_bytes[0])
print()

# callable()
# check if argument is callable
print("callable()")
def my_func():
    pass
class MyClass:
    def __call__(self):
        None
print("expect True:", callable(my_func))
print("expect True:", callable(lambda: None))
print("expect True:", callable(MyClass()))
print("expect False:", callable(1))
print()

# chr()
# string repr of character
print("chr()")
print(chr(33))
print()

# compile(), exec(), eval()
print("compile()", "exec()", "eval()", sep=", ")
code_string = "print('this should be six:', 1 + 2 + 3)"
# so compile takes a bunch of different modes apparently:
# 'exec', 'eval', 'single'
# I don't really care about the differences right now, can work it out later if I
# ever use this feature.
code_obj = compile(code_string, '<string>', 'exec')
exec(code_obj)

# eval has access to lexical scope
a = 1
b = 2
c = 10
print("this should be thirteen:", eval("a + b + c"))

print()

# complex()
print("complex()")
print(complex(5))
print(complex(5, 2))
print()

# attribute stuff
print("hasattr()", "getattr()", "setattr()", sep=", ")
class Empty:
    def __init__(self):
        self.a = 100
        self.b = "moose"
v = Empty()
print("hasattr('a'):", hasattr(v, 'a'))
print("hasattr('b'):", hasattr(v, 'b'))
delattr(v, 'a')
print("hasattr('a'):", hasattr(v, 'a'))
print("getattr('b'):", getattr(v, 'b'))
setattr(v, 'c', [1,2,3])
print("hasattr('c'):", hasattr(v, 'c'))
print("getattr('c'):", getattr(v, 'c'))
print("")

print("dict()")
print(dict(name="Jason", age="old AF"))
print(dict([('foo', 1), ('bar', 2)]))
print("")

# scope inspection
# dir() seems to get symbol names in list
# locals() gets map of ident -> value
print("dir()", "globals()", "locals()", sep=", ")
print("dir()", dir())
print("globals()", globals())
print("locals()", locals())
def dir_func(a, b, c):
    print("dir() inside func:", dir())
    print("locals() inside func:", locals())
dir_func(1, 2, 3)
print()

# divmod()
print("divmod()")
q, r = divmod(15, 6)
print("quotient", q)
print("remainder", r)
print()

# enumerate()
# adds a counter to an iterable
# yields pairs containing count (starting from zero), and the corresponding value
print("enumerate()")
for i, item in enumerate([1, 2, 3, 4, 5]):
    print(f"ix: {i} val: {item}")
print()

# filter()
print("filter()")
def is_even(num):
    return num % 2 == 0
evens = filter(is_even, [1, 2, 3, 4, 5, 6])
print(evens) # this returns an iterable object
print(list(evens)) # list() gets the actual list
# note - the filter object is "one shot"; once its been materialized via list(), you can't
# iterate over it again

# pass None to filter to make it remove all falsey items
truthy = filter(None, ["", [], {}, "these", "should", "stay", False])
for v in truthy:
    print(v)

print()

# type conversions:
# float(), int(), str(), bool()
print("bool()")
print("expect False:", bool(""))
print("expect True:", bool([1, 2, 3]))
print()

# format()
# i imagine there's a lot more to all of this in terms of customising format strings
# learn it when we need it...
print("format()")
print("my name is {0}".format("Jason"))
print()

# set(), frozenset()
print("set()", "frozenset()", sep=", ")
print(set([1, 2, 3]))
print(frozenset([4, 5, 6])) # immutable - suitable for use a dictionary key, or element in another set
print()

# hash()
# compute hash code of object
# only works on "hashable()" objects - that is, immutable types
# string, number, tuple, frozenset, etc
print("hash()")
print("1:", hash(1))
print("frozenset:", hash(frozenset([1, 2, 3, 4])))
print("tuple:", hash((1, 2, 3, True)))
print("string:", hash("this is a string to hash"))
print("None:", hash(None))
print("True:", hash(True))
print()

# help()
# invoke built-in help system
# skip this...

# hex(), oct()
# convert int to lowercase hex/oct string
print("hex()", "oct()", sep=", ")
print("hex:", hex(255))
print("oct:", oct(12))
print()

# id()
# return "identity" of object - unique int that identifies object in memory
print("id()")
print(id([1,2,3]))
print()

# input()
# get input from user
# skip...

# isinstance(), issubclass()
print("isinstance()", "issubclass()", sep=", ")
class MyClass:
    pass
class MySubClass(MyClass):
    pass
o = MyClass()
print(isinstance(o, MyClass))
print(issubclass(MySubClass, MyClass))
print()

# iter(), next()
print("iter()", "next()", sep=", ")
lst = [1, 2, 3, 4]
it = iter(lst)
print(next(it))
print(next(it))
print(next(it))
print(next(it))
print()

# len()
print("len()")
print(len("hello!"))
print(len([1,2,3]))
print(len({a: 1, b: 2}))
print()

# list()
print("list()")
lst = list()
lst.append(100)
lst.append(200)
lst.append(300)
print(lst)
print()

# map()
print("map()")
lst = [1,2,3]
def double(x):
    return x * 2
print(list(map(double, lst)))
print()

# min(), max()
print("min()", "max()", sep=", ")
print("min", min([1, 2, 3, 4, 5]))
print("max", max([1, 2, 3, 4, 5]))
print("min", min(1, 10, 100, 1000))
print("max", max(1, 10, 100, 1000))
print()

# object()
# creates an empty object
# object is the base for all classes in python
print("object()")
o = object()
print(o)
print()

# open()
print("open()")
lines = 0
with open("NOTES.md", mode='r') as file:
    while file.readline():
        lines += 1
print("lines in NOTES.md:", lines)
print()

# ord()
# get unicode code point
print("ord()")
print(ord('A'))
print(ord('🚗'))
print()

# pow(base, exp, [mod])
# optional third argument is mod, equiv to (base ** exp) % mod, but more efficient
print("pow()")
print(pow(2, 10)) # => 1024
print(pow(2, 10, 1000)) # => 24
print()

# print()
# yep...

# property()
# bit advanced this one, it allows custom getters/setters
print("property()")
class P:
    def __init__(self, name):
        self._name = name
    
    def get_name(self):
        print("getting name...")
        return self._name
    
    name = property(get_name)
p = P("Jason")
print(p.name)
print()

# repr()
# returns a string containing printable version of object
# goal is to be unambiguous, not necessary human readable
print("repr()")
print(repr("hello"))
print(repr(12))
print(repr(12.0))
print(repr(True))
print()

# reversed()
print("reversed()")
print(list(reversed([1,2,3])))
print()

# round()
print("round()")
print(round(3.14))
print(round(4.8))
print(round(2.5)) # round half to even
print(round(3.5)) # round half to even
print()

# slice()
print("slice()")
animals = ['zebra', 'sloth', 'giraffe', 'leopard']
print(animals[0:4])
print(animals[2:4])
print(animals[2:2])
print(animals[1:2])
print(animals[1:])
print(animals[:])
print(animals[0:-1])
print(animals[0:-2])

s = animals[0:2]
s[0] = "bro"
print(s)
print(animals)

print(id(animals))
print(id(animals[:]))

print()

# sorted()
print("sorted()")
numbers = [2,6,1,3,2,1]
print(sorted(numbers))
print(sorted(numbers, reverse=True))
print()

# str()
print("str()")
print(str(1.2))
print(str(True))
print(str(MyClass))
print()

# sum()
print("sum()")
print(sum([1,2,3,4,5]))
print(sum(iter([1,2,3,4,5])))
print()

# super()
# returns a proxy thing for calling methods on parent instance
print("super()")
class Parent:
    def foo(self):
        print("Parent foo")
class Child(Parent):
    def foo(self):
        super().foo()
        print("Child foo")
Child().foo()
print()

# tuple()
print("tuple()")
t = tuple(sorted([4,7,1]))
print(t)
print()

# type()
print("type()")
print(type("str"))
print(type(123))
print(type(1.21))
print(type(True))
print(type(False))
print(type([1,2,3]))
print(type(iter([1,2,3])))
print(type(set([1,2,3])))
print()

# vars()
print("vars()")
class Boo:
    def __init__(self):
        self.a = 1
        self.b = 2
        self.c = 3
class Coo(Boo):
    def __init__(self):
        super().__init__()
        self.d = 4
        self.e = 5
print(vars(Coo()))
print()

# zip()
# Aggregate multiple iterables into a single iterator of tuples
# stops when the shortest is exhausted
print("zip()")
names = ['Jason', 'Bob', 'Carla', 'Roberto']
ages = [44, 39, 50]
towns = ['Jazville', 'Bobville', 'Carlaville']
for n, a, t in zip(names, ages, towns):
    print(f"{n} is {a} and lives in {t}")
print(list(zip(names, ages, towns)))
print()

# import()
# imports libraries

# skipping these until we're a bit more advanced:
# classmethod() - transform a method into a class method
# staticmethod() - transform a method into a static method