
class MyClass:
    def __init__(self):
        self._protected = 10
        self.__private = 20

obj = MyClass()

# the single underscore thing is just a convention; not enforced by language
print(obj._protected)

# (although import * will not import names starting with _)


class Animal:
    def __init__(self, name):
        self.name = name
    
    def speak(self):
        print("<inaudible>")

class Dog(Animal):
    def speak(self):
        print("woof")

class Cat(Animal):
    def speak(self):
        print("meow")

Dog("jake").speak()
Cat("ruby").speak()
