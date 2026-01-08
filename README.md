# 8-Bit CPU Emulator

## CPU Architecture
- Word Size
    - **Data Width:** 8 bits
    - **Address width:** 16 bits
    - **Address space:** 64 KB (0x0000-0xFFFF)

## Supported Instructions

| Instruction | Syntax (Reg-Reg) | Syntax (Reg-Imm) |
| ----------- | ---------------- | ---------------- |
| MOV         | mov dest, src    | mov reg, imm     |
| ADD         | add r1, r2       | add reg, imm     |
| SUB         | sub r1, r2       | sub reg, imm     |
| JMP         | jmp addr         | jmp addr         |
| JZ          | jz addr          | jz addr          |
| JNZ         | jnz addr         | jnz addr         |
| CMP         | cmp reg, reg     | cmp reg, imp     |
| HLT (Halt)  | hlt              | hlt              |

## Registers
| Register | Size   | Description                    |
| -------- | ------ | ------------------------------ |
| A        | 8-bit  | General                        |
| B        | 8-bit  | General                        |
| C        | 8-bit  | General                        |
| D        | 8-bit  | General                        |
| PC       | 16-bit | Program Counter                |
| SP       | 16-bit | Stack pointer (unused for now) |

## Flags
| Flag  | Description  |
| ----- | ------------ |
| Z     | Zero Flag    |
| C     | Carry/Borrow |


# Usage
```bash
cargo run -- --f <examples/filename.asc>
```

## Todo
- [x] Assembler
    - [x] Lexer/Tokenizer
    - [x] Add label support (supporting JMP/JZ/JNZ)
- [ ] Add instructions
    - [x] CMP
    - [x] MUL
    - [x] DIV
    - [ ] CALL
    - [ ] RET
- [ ] Error handling
- [ ] Build Debugger
