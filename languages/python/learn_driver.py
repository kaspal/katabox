{{sample}}

# --- KataBox Learn sample check (ADR 0029): read the values, call the sample on the
# array `a` (the call may also set `a` to the sample's result), print it. kb-pack wraps sample.py in this; never shown in the app.
import sys as _kb_sys

a = [int(_kb_token) for _kb_token in _kb_sys.stdin.read().split()]
{{call}}
print(" ".join(str(_kb_value) for _kb_value in a))
