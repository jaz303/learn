def my_func(*args, **kwargs):
    print("args", args)
    print("kwargs", kwargs)

my_func("a", 1, 2, True, foo="bar", baz="bleem")