# context manager - object that is notified when a context (block of code)
# starts and ends.
#
# the "with" statement takes care of these notifications

# file objects are context managers...
# with open(filename) as f

# here's a custom ContextManager

class ContextManager:
    def __enter__(self, *args, **kwargs):
        print("--enter--")

    def __exit__(self, *args):
        print("--exit--")

with ContextManager():
    print("test")

# it's also possible to use the stdlib library contextlib which has
# decorators... looks a bit complicated for now... come back to it.