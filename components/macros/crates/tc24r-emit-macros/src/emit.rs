//! Assembly emission macros.

/// Emit a single line of assembly to the output buffer.
///
/// Appends the line followed by a newline to `$state.out`.
///
/// # Usage
///
/// ```rust
/// # use tc24r_emit_macros::emit;
/// # struct State { pub out: String }
/// # let mut state = State { out: String::new() };
/// // Emit a literal string:
/// emit!(state, "        add     r0,r1");
///
/// // Emit with format arguments:
/// let offset = -3;
/// emit!(state, "        lw      r0,{offset}(fp)");
///
/// // Emit with explicit format:
/// let name = "main";
/// emit!(state, "        la      r0,_{name}");
/// # assert_eq!(state.out, "        add     r0,r1\n        lw      r0,-3(fp)\n        la      r0,_main\n");
/// ```
#[macro_export]
macro_rules! emit {
    ($state:expr, $lit:literal) => {
        $state.out.push_str(&format!($lit));
        $state.out.push('\n');
    };
    ($state:expr, $fmt:literal, $($arg:tt)*) => {
        $state.out.push_str(&format!($fmt, $($arg)*));
        $state.out.push('\n');
    };
}

/// Emit multiple lines of assembly in one call.
///
/// Each argument is a line to emit. Reduces visual noise when emitting
/// a sequence of instructions.
///
/// # Usage
///
/// ```rust
/// # use tc24r_emit_macros::emit_lines;
/// # struct State { pub out: String }
/// # let mut state = State { out: String::new() };
/// emit_lines!(state,
///     "        push    fp",
///     "        push    r2",
///     "        push    r1",
///     "        mov     fp,sp",
/// );
/// # assert!(state.out.contains("push    fp"));
/// # assert!(state.out.contains("mov     fp,sp"));
/// ```
#[macro_export]
macro_rules! emit_lines {
    ($state:expr, $($line:literal),+ $(,)?) => {
        $(
            $state.out.push_str($line);
            $state.out.push('\n');
        )+
    };
}

/// Emit a label definition (no indentation).
///
/// # Usage
///
/// ```rust
/// # use tc24r_emit_macros::emit_label;
/// # struct State { pub out: String }
/// # let mut state = State { out: String::new() };
/// let name = "L0";
/// emit_label!(state, name);
/// # assert_eq!(state.out, "L0:\n");
/// ```
#[macro_export]
macro_rules! emit_label {
    ($state:expr, $name:expr) => {
        $state.out.push_str(&format!("{}:", $name));
        $state.out.push('\n');
    };
}

/// Emit an assembly comment line.
///
/// # Usage
///
/// ```rust
/// # use tc24r_emit_macros::emit_comment;
/// # struct State { pub out: String }
/// # let mut state = State { out: String::new() };
/// emit_comment!(state, "prologue");
/// # assert_eq!(state.out, "; prologue\n");
/// ```
#[macro_export]
macro_rules! emit_comment {
    ($state:expr, $text:literal) => {
        $state.out.push_str(&format!("; {}", $text));
        $state.out.push('\n');
    };
}
