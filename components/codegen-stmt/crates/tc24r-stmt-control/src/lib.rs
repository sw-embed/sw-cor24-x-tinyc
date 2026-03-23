//! Control-flow statement code generation (If, While, DoWhile, For).

mod dowhile_stmt;
mod for_stmt;
mod if_stmt;
mod switch_stmt;
mod while_stmt;

pub use dowhile_stmt::gen_do_while;
pub use for_stmt::gen_for;
pub use if_stmt::gen_if;
pub use switch_stmt::gen_switch;
pub use while_stmt::gen_while;
