# Introduction

...

## MVP

- Just query
- Support the basic structures for PCRE
    - Letters (uppercase, lowercase)
    - Numbers
    - Whitespaces
    - Groups
    - Globbing (., *)

## Examples

> REGEXER_LANGUAGE => REGEX RESULT

```
.letter(upcase).glob(rest=True).whitespace().number() => [A-Z].*\s[0-9]
.letters(upcase).glob(rest=True).whitespace().numbers() => [A-Z]+.*\s[0-9]+
.group(letters(upcase).glob(rest=True)).whitespace().group(numbers()) => ([A-Z]+.*)\s([0-9]+)
```
