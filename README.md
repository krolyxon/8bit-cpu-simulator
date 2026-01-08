# 8-Bit CPU Emulator

## CPU Architecture
- Word Size
    - **Data Width:** 8 bits
    - **Address width:** 16 bits
    - **Address space:** 64 KB (0x0000-0xFFFF)

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
| HLT (Halt)  | hlt              |


## Registers
| Register | Size   | Description                    |
| -------- | ------ | ------------------------------ |
| A        | 8-bit  | General                        |
| B        | 8-bit  | General                        |
| C        | 8-bit  | General                        |
| D        | 8-bit  | General                        |
| PC       | 16-bit | Program Counter                |
| SP       | 16-bit | Stack pointer                  |

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
    - [x] CALL
    - [x] RET
- [ ] Error handling
- [ ] Build Debugger
