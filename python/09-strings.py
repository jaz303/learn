print("this is\na normal \"string\"")
print(r"this is \nraw \"string\"")

print(
"""
this
is
a
multline
string"""
)

print('hell' in 'hello')
print('foo' in 'bar')

print('hello world'.upper())
print('hello world'.lower())
print('hello world'.title())

# isupper()
# islower()
# isalpha()
# isalnum()
# isdecimal()
# isspace()
# istitle()
# startswith()
# endswith()

print(", ".join(["a", "b", "c"]))

print("a b c".split())
print("a,b,c".split(","))

print("foo".rjust(30))
print(repr("bar".ljust(30, "~")))
print(repr("bleem".center(50, "=")))

# strip()
# lstrip()
# rstrip()

string = "this is a sentence"
print(string.count("e"))
print(string.count("this"))

# str.replace(old, new, count=1)