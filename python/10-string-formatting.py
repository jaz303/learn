from datetime import datetime

# old way
name = "Pete"
print("Hello %s" % name)

name = "Jason"
print(f"Hello {name}")

now = datetime.now().strftime("%b/%d/%Y - %H:%M:%S")
# = -> print the expression and its value
print(f"This time is {now=}")

a = 10_000
print(f"{a:,}")

a = 3.1415926
print(f"{a:.2f}")

a = 0.751231234
print(f"{a:.2%}")

# features:
# - decimal places (with/without sign)
# - zero-padding (left/right)
# - character-padding (left/right)
# - number format (e.g. thousands)
# - percentages
# - scientific notation
# - right/left aligned

# there's a "Template strings" library that allows stuff like foo = Template("Hey $name!"); foo.substitute(name="Jason")