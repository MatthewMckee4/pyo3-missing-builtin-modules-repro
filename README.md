# Repro

To setup and see the issue run:

```
uv venv
uv pip install pytest
# Activate virtual env, here we can use uv to activate it.
uv run cargo test
```

This will run in a failing test as `import pytest` fails.

I *think* the issue is that some buitin modules like `_decimal` are missing.

This is the traceback from the failing test:

```
Traceback (most recent call last):
  File "<string>", line 1, in <module>
  File "/home/matthew/develop/personal/pyo3-missing-builtin-modules-repro/.venv/lib/python3.13/site-packages/pytest/__init__.py", line 8, in <module>
    from _pytest._code import ExceptionInfo
  File "/home/matthew/develop/personal/pyo3-missing-builtin-modules-repro/.venv/lib/python3.13/site-packages/_pytest/_code/__init__.py", line 5, in <module>
    from .code import Code
  File "/home/matthew/develop/personal/pyo3-missing-builtin-modules-repro/.venv/lib/python3.13/site-packages/_pytest/_code/code.py", line 9, in <module>
    import dataclasses
  File "/home/matthew/.local/share/uv/python/cpython-3.13.0-linux-x86_64-gnu/lib/python3.13/dataclasses.py", line 5, in <module>
    import inspect
  File "/home/matthew/.local/share/uv/python/cpython-3.13.0-linux-x86_64-gnu/lib/python3.13/inspect.py", line 146, in <module>
    import dis
  File "/home/matthew/.local/share/uv/python/cpython-3.13.0-linux-x86_64-gnu/lib/python3.13/dis.py", line 8, in <module>
    from opcode import *
  File "/home/matthew/.local/share/uv/python/cpython-3.13.0-linux-x86_64-gnu/lib/python3.13/opcode.py", line 12, in <module>
    import _opcode
```
