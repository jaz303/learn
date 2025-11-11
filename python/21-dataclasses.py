from dataclasses import dataclass
from typing import Any
from collections import namedtuple

@dataclass
class Person:
    name: str
    age: int
    metadata: Any = None
    food: str = "anything"

jason = Person("Jason", age=44, metadata={"foo": 123})
print(jason)
print(repr(jason))
print(jason.name)

# see also: namedtuple
# useful for CSV, sqlite, etc...
Point = namedtuple('Point', ['x', 'y'])
p1 = Point(x=100, y=200)
print(p1)
print(p1[0])
print(p1.x)
x, y = p1
print(x, y)