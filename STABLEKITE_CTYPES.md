# Recognize native ctypes output conversion

Ruff 0.17.4+stablekite.1 recognizes `__ctypes_from_outparam__` as a known
Python protocol method. CPython's `ctypes` module invokes this method when
converting output arguments from native function calls. Defining the method on
a `ctypes.Structure` or scalar subclass must not trigger PLW3201.

The protocol is present in the primary CPython sources for
[Python 3.13](https://github.com/python/cpython/blob/v3.13.13/Modules/_ctypes/_ctypes.c)
and [Python 3.15](https://github.com/python/cpython/blob/v3.15.2/Modules/_ctypes/_ctypes.c).

Only the exact protocol name is recognized. Misspellings and invented method
names continue to trigger PLW3201. Explicit calls to this hook retain their
previous behavior under PLC2801: no Python operator or built-in function
replaces the conversion. Other unnecessary dunder calls remain diagnosed.

The regression fixture covers structure, scalar, and inherited definitions,
seven invalid names, the conversion call, and an unnecessary `__str__` call.
