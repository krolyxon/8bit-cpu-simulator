# 8-Bit CPU Emulator

This project is an educational 8-bit CPU emulator written in Rust, it's build to understand core computer architecture concepts such as instruction sets, flags,
control flow, stacks and system calls.

This project is **not** a production CPU and **not** accurate. It is designed purely for learning and experimentation.

-----

## Features

- Custom 8-bit ISA (control-flow and arithmetic operations)
- 4 general-purpose registers
- 64 KB (0x0000-0xFFFF) byte-addressable memory
- Two-pass assembler with label support
- Stack-based function calls
-  Stack is software-managed and grows downward

---



## Registers
| Register | Size   | Description                    |
| -------- | ------ | ------------------------------ |
| A        | 8-bit  | General-purpose register                        |
| B        | 8-bit  | General-purpose register                        |
| C        | 8-bit  | General-purpose register                        |
| D        | 8-bit  | General-purpose register                        |
| PC       | 16-bit | Program Counter                |
| SP       | 16-bit | Stack pointer                  |


## Flags Register

The CPU maintains a small flags register.

| Flag  | Description  |
| ----- | ------------ |
| Z     | Zero flag - set if last result was `0`   |
| C     | Carry/Borrow flag |

- Control-flow instructions do **not** modify flags.

## Supported Instructions

| Instruction | Syntax |
| ----------- | ---------------- |
| MOV         | mov dest, src  OR mov reg, imm     |
| ADD         | add r1, r2     OR add reg, imm     |
| SUB         | sub r1, r2     OR sub reg, imm     |
| JMP         | jmp addr       OR jmp addr         |
| JZ          | jz addr        OR jz addr          |
| JNZ         | jnz addr       OR jnz addr         |
| CMP         | cmp r1, r2     OR cmp reg, imp     |
| MUL         | mul r1, r2     |
| DIV         | div r1, r2     |
| CALL        | call \<label\>     |
| SYS         | sys \<syscall_no\> |
| HLT (Halt)  | hlt                                |

## Syscalls

| Imm | Meaning |
| --- | ------- |
| 0   | Exit Program |
| 1   | Print register A as integer |
| 2   | Print register A as ASCII char |

- Syscalls also do not modify flags

## Example assembly program

```assembly
mov b, 3
mov a, 1

loop:
    sub b, a
    jnz loop

sys 1   ; print register A
sys 0   ; exit
```

## Usage

```bash
cargo run -- --f <examples/filename.asc>
```

## Non-goals

1. No pipelining
2. No interrupts
3. No virtual memory
4. No privilege levels
5. No hardware I/O

## Goals

- [x] Assembler
    - [x] Lexer/Tokenizer
    - [x] Add label support (supporting JMP/JZ/JNZ)
- [x] Add instructions
    - [x] CMP
    - [x] MUL
    - [x] DIV
    - [x] CALL
    - [x] RET
    - [x] SYS
- [ ] Better error-handling
- [ ] TUI debugger
