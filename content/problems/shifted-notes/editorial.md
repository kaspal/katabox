## Approach: decode the text block in one pass

**Reading.** The first line is the key; everything after it is text of unknown length, and its exact layout matters: leading spaces, punctuation and empty lines all have to come out unchanged. So do not split it into tokens. Read the whole input at once, cut it at the first line break, parse the part before it as `s` and keep the rest as one block.

**Decoding a letter.** Encoding moved each letter forward `s` places, so decoding moves it back `s` places in a 26-letter cycle. For a lowercase letter `c`:

```
p = c - 'a'                 # 0..25
plain = 'a' + (p - s + 26) mod 26
```

and the same with `'A'` for uppercase letters. The `+ 26` keeps the value non-negative in languages where `%` of a negative number is negative. Any other character, including the line breaks, is copied as is, so empty lines survive on their own.

**One table instead of arithmetic.** There are only 52 letters, so a translation table from each encoded letter to its plain letter can be built once: the plain alphabet is the alphabet rotated by `s`. Python's `bytes.maketrans` + `translate` and PHP's `strtr` then decode the whole block in a single call, in C.

**Why it is correct:** shifting forward by `s` and back by `s` within the same 26-letter cycle cancel out, the letter case never changes, and everything else, line structure included, is untouched.

**Complexity:** O(L) time and memory for L characters of text.

**Fast input in each language:** `sys.stdin.buffer.read()` with `partition(b"\n")` and `translate` (Python), `readFileSync(0)` as a `Buffer`, changed in place and written back with `process.stdout.write` (TypeScript), `stream_get_contents(STDIN)` with `strtr` (PHP), `read_to_end` into a `Vec<u8>`, mapped byte by byte and written with `write_all` (Rust).
