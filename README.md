# MetaBrainFuck
This is BrainFuck with some convenience features:
- The ability to split code into multiple files
- A macro to repeat code

As you can see, the list is very small. Feel free to make feature requests!

I didn't want to add an interpreter to let the user choose one.

## Usage
1. Download the executable from the latest release or compile from source with `cargo build`.
2. Run `mbf [input.mbf] [output.bf]` (or mbf.exe if you're using Windows).

    Both arguments are optional and default to `main.mbf` and `out.bf`.

## Example

main.mbf
```bf
Normal BrainFuck
[]<>+-,.


Include MetaBrainFuck and BrainFuck files
test.mbf
test.bf


Repeat code
repeat(test.mbf; 3)

repeat(test.mbf
+++
test.mbf
; 3)
```
test.mbf
```bf
.-.
```
test.bf
```bf
-.-
```
Output:
```bf
Normal BrainFuck
[]<>+-,.


Include MetaBrainFuck and BrainFuck files
.-.
-.-


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
```