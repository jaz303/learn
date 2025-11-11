# decorator is function that takes a function and returns a wrapper

def my_decorator(fn):
    def wrapper(*args, **kwargs):
        print("Before!")
        fn(*args, **kwargs)
        print("After!")
    return wrapper

@my_decorator
def foo(name):
    print(f"hey {name} this is cool")

foo("Jason")
foo(name="Jason")

# decorators can take arguments

# ignoring the stuff on class decorators, think it's better to read the manual