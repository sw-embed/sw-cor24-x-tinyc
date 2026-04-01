# Feature Request: Function Pointer Support

**Priority:** High
**Requested by:** sw-cor24-script (sws interpreter)

## Summary

tc24r does not currently support function pointers. Function pointers are part
of standard C (C89/C90 and all later versions) and are needed for idiomatic
command dispatch tables, callbacks, and similar patterns.

## Motivation

The sws scripting language interpreter needs a command dispatch table mapping
command name strings to handler functions. The natural C pattern is:

```c
typedef void (*cmd_handler_t)(int argc, char **argv);

cmd_handler_t handlers[MAX_CMDS];
char cmd_names[MAX_CMDS][MAX_NAME];
int cmd_count;

void dispatch(char *name, int argc, char **argv) {
    for (int i = 0; i < cmd_count; i++) {
        if (str_eq(cmd_names[i], name)) {
            handlers[i](argc, argv);
            return;
        }
    }
}
```

Without function pointers, the workaround is an integer-indexed switch
statement, which is fragile, harder to extend, and non-idiomatic.

## Required Capabilities

At minimum, the following should work:

### 1. Function pointer variables

```c
int (*fp)(int, int);
fp = add;
int result = fp(3, 4);
```

### 2. Arrays of function pointers

```c
int (*table[8])(int);
table[0] = handler_echo;
table[1] = handler_set;
table[n](arg);
```

### 3. Function pointers as parameters

```c
void apply(int (*f)(int), int x) {
    f(x);
}
```

### 4. Typedef for function pointer types (nice to have)

```c
typedef int (*handler_t)(int);
handler_t table[8];
```

## COR24 Implementation Notes

- On COR24, function pointers are 24-bit values (same as `int` and all other
  pointers) — they hold code addresses
- Calling through a function pointer requires an indirect jump/call
  instruction (e.g., `call (R1)` or equivalent in the ISA)
- The compiler needs to emit the function's address as a loadable constant
  when taking `&func` or using a bare function name in pointer context
- Storage in arrays works identically to other pointer arrays

## ISA Consideration

Verify that the COR24 ISA supports indirect calls (call through a register).
If not, that is a prerequisite at the emulator/ISA level. A `call (Rn)`
or `jsr (Rn)` instruction is needed.

## Workaround Until Implemented

The sws interpreter will use integer handler indices with a switch-based
dispatcher:

```c
#define CMD_ECHO 0
#define CMD_SET  1

int cmd_handler[MAX_CMDS]; /* handler index */

switch (cmd_handler[i]) {
    case CMD_ECHO: cmd_echo(argc, argv); break;
    case CMD_SET:  cmd_set(argc, argv); break;
}
```

This works but scales poorly and requires updating the switch for every new
command.
