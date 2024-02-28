# MetaBrainFuck
This is BrainFuck with some convenience features:
- The ability to split code into multiple files;
- Simpler numbers;
- A macro to repeat code;
- A macro to map code to other code.

As you can see, the list is very small. Feel free to make feature requests!

This "compiler" "compiles" MetaBrainFuck to regular BrainFuck.

MBF files have the .mbf extension.

I didn't want to add an interpreter to let the user choose one.

## Features

### Use multiple files
Just type the name of an MBF or BF file and it will be replaced with that file's contents!

The current directory is scanned recursively for MBF and BF files.

Example:

main.mbf
```bf
+++
test.mbf
<
```

test.mbf
```bf
--->
```

Output:
```bf
+++
--->
<
```

### Simpler numbers
You can prefix a number with a + or - to have that amount of + or -.

Example:

main.mbf
```bf
+42
-6
```

Output:
```bf
++++++++++++++++++++++++++++++++++++++++++
------
```

### Repeating code
You can use `repeat(CODE; N)` to repeat `CODE` N times. `CODE` can literally be anything.

Example:

main.mbf
```bf
repeat(-.-; 3)

repeat(.-.
+++ Even comments!
.-.
; 3)
```

Output:
```bf
-.--.--.-

.-.
+++ Even comments!
.-.
.-.
+++ Even comments!
.-.
.-.
+++ Even comments!
.-.

```

### Mapping code
With this macro you can map any code to any other code.
Keep in mind that all mappings are applied the entirety of the program, so it doesn't matter where you call this macro.
Usage: `map(CODE; CODE)`

Example:

main.mbf
```bf
map(CODE; +++
---
+++)
CODE
```

Output:
```bf

+++
---
+++
```

## Usage
1. Download the executable from the latest release or compile from source with `cargo build`.
2. Run `mbf [(-c | --charset) CHARSET] INPUT OUTPUT` (or mbf.exe if you're using Windows).

    If `-c` or `--charset` is used, the compiler will discard any character not present in `CHARSET`.
    Both `INPUT` and `OUTPUT` are optional and default to `main.mbf` and `out.bf`.

## Full example

main.mbf
```bf
Normal BrainFuck
[]<>+-,.


Include MetaBrainFuck and BrainFuck files
test.mbf
test.bf


Write numbers
+42
-6


Repeat code
repeat(test.mbf; 3)

repeat(test.mbf
+++
test.mbf
; 3)


Map code to other code
map(CODE; +++
---
+++)
CODE
```

test.mbf
```bf
.-.
```

test.bf
```bf
-.-
```

Output
```bf
Normal BrainFuck
[]<>+-,.


Include MetaBrainFuck and BrainFuck files
.-.
-.-


Write numbers
++++++++++++++++++++++++++++++++++++++++++
------


Repeat code
.-..-..-.

.-.
+++
.-.
.-.
+++
.-.
.-.
+++
.-.



Map code to other code

+++
---
+++
```
