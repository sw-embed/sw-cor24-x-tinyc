//! Basic emit and label helpers.

use cor24_isa::branch::can_short_branch;
use tc24r_codegen_state::CodegenState;

/// Append one line of assembly to the output buffer.
/// Also increments the instruction counter (for branch range estimation)
/// and auto-records label positions for branch range checking.
pub fn emit(state: &mut CodegenState, line: &str) {
    state.out.push_str(line);
    state.out.push('\n');
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return;
    }
    if trimmed.ends_with(':') {
        // Label line — record its position (strip the colon)
        let label = &trimmed[..trimmed.len() - 1];
        state
            .label_positions
            .insert(label.to_string(), state.instruction_count);
    } else {
        // Instruction line
        state.instruction_count += 1;
    }
}

/// Allocate a fresh unique label and return it.
pub fn new_label(state: &mut CodegenState) -> String {
    let label = format!("L{}", state.label_counter);
    state.label_counter += 1;
    label
}

/// Emit an unconditional branch to target.
///
/// Uses short `bra` when the target is within short-branch range
/// (based on instruction count distance). Falls back to long form
/// (la r2 + jmp) for far targets.
pub fn emit_bra(state: &mut CodegenState, target: &str) {
    if is_short_branch(state, target) {
        emit(state, &format!("        bra     {target}"));
    } else {
        emit(state, &format!("        la      r2,{target}"));
        emit(state, "        jmp     (r2)");
    }
}

/// Emit a branch-if-true (carry flag set) to target.
pub fn emit_brt(state: &mut CodegenState, target: &str) {
    if is_short_branch(state, target) {
        emit(state, &format!("        brt     {target}"));
    } else {
        let skip = new_label(state);
        emit(state, &format!("        brf     {skip}"));
        emit(state, &format!("        la      r2,{target}"));
        emit(state, "        jmp     (r2)");
        emit(state, &format!("{skip}:"));
    }
}

/// Emit a branch-if-false (carry flag clear) to target.
pub fn emit_brf(state: &mut CodegenState, target: &str) {
    if is_short_branch(state, target) {
        emit(state, &format!("        brf     {target}"));
    } else {
        let skip = new_label(state);
        emit(state, &format!("        brt     {skip}"));
        emit(state, &format!("        la      r2,{target}"));
        emit(state, "        jmp     (r2)");
        emit(state, &format!("{skip}:"));
    }
}

/// Determine if a short branch can safely reach the target.
///
/// For backward branches (target already emitted), checks the instruction
/// distance using cor24-isa's conservative range estimate.
/// For forward branches (target not yet emitted), uses short form optimistically
/// since forward branches within a single control-flow construct are typically close.
/// Global/function labels (starting with '_') always use long form.
fn is_short_branch(state: &CodegenState, target: &str) -> bool {
    // Global labels (e.g., _main, _halt) are potentially far — always long
    if target.starts_with('_') {
        return false;
    }

    // If the label has already been emitted, check actual distance
    if let Some(&label_pos) = state.label_positions.get(target) {
        return can_short_branch(state.instruction_count, label_pos);
    }

    // Forward reference to a local label — use long branch to be safe.
    // We don't know the distance yet, and large function bodies (like
    // tml24c's eval) can exceed the ±127 byte short branch range.
    // This is conservative (small functions get slightly larger output)
    // but never produces assembler errors.
    false
}
