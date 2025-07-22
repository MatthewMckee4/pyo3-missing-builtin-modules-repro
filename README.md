# Repro

To setup and see the issue run:

```
uv venv
uv pip install pytest
cargo test
```

This will run in a failing test as `import pytest` fails.

I *think* the issue is that some buitin modules like `_decimal` are missing.

This is the traceback from the failing test:

```
Traceback (most recent call last):
  File "<string>", line 1, in <module>
  File "/Developer/pyo3-missing-builtin-modules-repro/.venv/lib/python3.13/site-packages/pytest/__init__.py", line 24, in <module>
    from _pytest.doctest import DoctestItem
  File "/Developer/pyo3-missing-builtin-modules-repro/.venv/lib/python3.13/site-packages/_pytest/doctest.py", line 41, in <module>
    from _pytest.python_api import approx
  File "/Developer/pyo3-missing-builtin-modules-repro/.venv/lib/python3.13/site-packages/_pytest/python_api.py", line 8, in <module>
    from decimal import Decimal
  File "/.local/share/uv/python/cpython-3.13.1-linux-x86_64-gnu/lib/python3.13/decimal.py", line 105, in <module>
    import _pydecimal
  File "/.local/share/uv/python/cpython-3.13.1-linux-x86_64-gnu/lib/python3.13/_pydecimal.py", line 343, in <module>
    import contextvars
  File "/.local/share/uv/python/cpython-3.13.1-linux-x86_64-gnu/lib/python3.13/contextvars.py", line 1, in <module>
    from _contextvars import Context, ContextVar, Token, copy_context
```