# MetaBrainFuck
A BrainFuck preprocessor with some convenience features:
- The ability to split code into multiple files;
- Writing numbers;
- A macro to write ASCII strings;
- A macro to repeat code;
- A macro to map code to other code.

As you can see, the list is very small. Feel free to make feature requests!

MBF files have the .mbf extension.

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

### Writing numbers
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

### Writing strings
To write a string use `string("STRING")`. You don't need to escape any characters, unless you have `")`, in which case you need to escape it like so: `\")`.
This macro will write some bytes according to the length of the string. The cursor position remains unchanged.

Example:

main.mbf
```bf
string("Hello, World!")
```

Output:
```bf
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ >
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ >
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ >
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ >
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ >
++++++++++++++++++++++++++++++++++++++++++++ >
++++++++++++++++++++++++++++++++ >
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ >
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ >
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ >
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ >
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ >
+++++++++++++++++++++++++++++++++ >
<<<<<<<<<<<<<
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


Write strings
string("Hello, World!")


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


Write strings
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ >
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ >
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ >
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ >
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ >
++++++++++++++++++++++++++++++++++++++++++++ >
++++++++++++++++++++++++++++++++ >
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ >
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ >
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ >
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ >
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ >
+++++++++++++++++++++++++++++++++ >
<<<<<<<<<<<<<


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
