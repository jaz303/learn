# Data Types

  - `int`
  - `float`
  - `complex`: `2+3j`, `complex(2, 3)`
  - `str`
  - `bool`: `True` or `False`
  - `NoneType` (e.g. `None`)
  - `list` - ordered, mutable
  - `dict` - key-value pairs
  - `tuple` - ordered, immutable
  - `set` - unordered, unique items

# Useful libraries

  - `pathlib`: path and filesystem manipulation
  - `shutil`: copying/move/rename files/folders
  - `json`
  - `traceback`: get backtrace (useful for exception handling)
  - `logging`: built in logging library
  - `abc`: infrastructure for abstract base classes
  
# Modules and Packages

## Module

any Python file `.py` that can be imported. Python assigns the module a name, which is usually the filename (without the `.py` extension)

## Package

A directory containing `__init__.py`.

Can contain multiple modules, or subpackages (subdirs with `__init__.py`)

Package name is directory name, each file inside is a package module.

# Authoring Packages

File org:

```
zap/
  __init__.py
  _state_machine.py
  _codec.py
  _helpers.py
```

Leading underscores prevent users from importing directly.

Then in `__init__.py`:

```python
# . is a relative import (bypass sys.path)
# can also use .., ... etc.
# or even ...foo.bar - 2 levels up then into foo package then into the bar module
# relative imports only work inside packages.
from .state_machine import StateMachine
from .codec import ArgCodec
from .helpers import helper1, helper2

__all__ = [
  "StateMachine",
  "ArgCodec",
  "helper1",
  "helper2"
]
```

This allows:

```python
from zap import StateMachine, helper1
```