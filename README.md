# Repro

To setup and see the issue run:

```
uv venv
uv pip install pytest
cargo test
```

This will run in a failing test as `import pytest` fails.

I *think* the issue is that some buitin modules like `_decimal` are missing.
