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

### 08/12/2023

The initial idea here is to make the first "language" that comes to my mind in a
javascripty way

```
.letter(upcase).glob(rest=True).whitespace().number() => [A-Z].*\s[0-9]
.letters(upcase).glob(rest=True).whitespace().numbers() => [A-Z]+.*\s[0-9]+
.group(letters(upcase).glob(rest=True)).whitespace().group(numbers()) =>
([A-Z]+.*)\s([0-9]+)
```

### 09/12/2023

The javascripty way is very hard to parse when it reaches the whole group
thingy, and now I'm thinking that too much parenthesis is really bad for a
string that will be written in a shell environment. So my thoughts is using the
pipe from bash to create a nicer syntax with more whitespace.

Also, every parameter inside each function should be like python `upcase=True`
because we can in the future support more data types, not only booleans.

```
letter(upcase=True) | glob(rest=True) | whitespace | number => [A-Z].*\s[0-9]
letters(upcase=True) | glob(rest=True) | whitespace | numbers =>
[A-Z]+.*\s[0-9]+ 
group(letters(upcase=True) | glob(rest=True)) | whitespace | group(numbers) =>
([A-Z]+.*)\s([0-9]+) 
```

**Possibilities for a group**:

```
group(letters(upcase=True))
```

```
group(letter | whitespace)
```

```
group(letter | glob(rest=True))
```

