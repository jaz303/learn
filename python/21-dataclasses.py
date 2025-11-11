from dataclasses import dataclass
from typing import Any

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