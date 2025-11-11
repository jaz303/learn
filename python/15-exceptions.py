try:
    print(15/0)
except ZeroDivisionError:
    print("you cannot divide by 0...")
finally:
    print("...and done")
    
print("")

for x in range(0, 10):
    try:
        if x % 2 == 0:
            print(x / 0)
        else:
            print('str' + x)
    except (ZeroDivisionError, TypeError) as error:
        print(error)

#
# Custom exceptions

class MyException(Exception):
    pass

try:
    raise(MyException("a message"))
except MyException as error:
    print(error)