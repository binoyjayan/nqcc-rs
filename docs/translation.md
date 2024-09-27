# Introduction

## Translation

### Translate to assembly

```
gcc -S -O -fno-asynchronous-unwind-tables -fcf-protection=none return_2.c
```

### Generate object code

```
gcc return_2.s -o return_2
```

## The compiler driver

### Preprocess source file

```
gcc -E -P <program.c> -o <program.i>
```

The option `-P` tells the preprocessor to not emit linemarkers.

### Compile preprocessed source file

Compile the preprocessed source file and create an assembly file
with a `.s` extension.

### Assemble and link

Assemble the `.s` file and link it produce an executable.

```
gcc <program.s> -o <program>
```

Delete the assembly file `program.s` when complete.

### Compile driver options

| Flag     | Description                                        |
| -------- | -------------------------------------------------- |
| --lex    | Scan token but do not parse or compile             |
| --parse  | Parse but stop before assembly operation           |
| --codegen| Perform scanning, parsing and assembly generation  |
|          | but stop before code emission                      |
| -S       | Emit assembly file but do not assemble or link it  |

Except for `-S`, none of the above options produce an output file.