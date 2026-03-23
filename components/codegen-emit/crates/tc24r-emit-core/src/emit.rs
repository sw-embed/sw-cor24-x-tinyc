//! Basic emit and label helpers.
//!
//! Branch strategy: emit short branches optimistically for all local labels.
//! After each function is generated, `resolve_branches()` checks deferred
//! forward branches against known label positions using cor24-isa range
//! constants, and expands any out-of-range branches to long form.

use cor24_isa::branch::can_short_branch;
use tc24r_codegen_state::{BranchKind, CodegenState, DeferredBranch};

/// Append one line of assembly to the output buffer.
/// Auto-detects label lines and records their positions for branch range checking.
pub fn emit(state: &mut CodegenState, line: &str) {
    state.out.push_str(line);
    state.out.push('\n');
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return;
    }
    if trimmed.ends_with(':') {
        let label = &trimmed[..trimmed.len() - 1];
        state
            .label_positions
            .insert(label.to_string(), state.instruction_count);
    } else {
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
pub fn emit_bra(state: &mut CodegenState, target: &str) {
    if target.starts_with('_') {
        // Global labels — always long form
        emit(state, &format!("        la      r2,{target}"));
        emit(state, "        jmp     (r2)");
        return;
    }
    // Backward branch — check distance now
    if let Some(&label_pos) = state.label_positions.get(target) {
        if can_short_branch(state.instruction_count, label_pos) {
            emit(state, &format!("        bra     {target}"));
        } else {
            emit(state, &format!("        la      r2,{target}"));
            emit(state, "        jmp     (r2)");
        }
        return;
    }
    // Forward branch — emit short optimistically, defer for validation
    let line = format!("        bra     {target}");
    let out_offset = state.out.len();
    emit(state, &line);
    state.deferred_branches.push(DeferredBranch {
        out_offset,
        line,
        instruction_count: state.instruction_count - 1, // emit() already incremented
        target: target.to_string(),
        kind: BranchKind::Bra,
    });
}

/// Emit a branch-if-true (carry flag set) to target.
pub fn emit_brt(state: &mut CodegenState, target: &str) {
    if target.starts_with('_') {
        emit_long_brt(state, target);
        return;
    }
    if let Some(&label_pos) = state.label_positions.get(target) {
        if can_short_branch(state.instruction_count, label_pos) {
            emit(state, &format!("        brt     {target}"));
        } else {
            emit_long_brt(state, target);
        }
        return;
    }
    let line = format!("        brt     {target}");
    let out_offset = state.out.len();
    emit(state, &line);
    state.deferred_branches.push(DeferredBranch {
        out_offset,
        line,
        instruction_count: state.instruction_count - 1,
        target: target.to_string(),
        kind: BranchKind::Brt,
    });
}

/// Emit a branch-if-false (carry flag clear) to target.
pub fn emit_brf(state: &mut CodegenState, target: &str) {
    if target.starts_with('_') {
        emit_long_brf(state, target);
        return;
    }
    if let Some(&label_pos) = state.label_positions.get(target) {
        if can_short_branch(state.instruction_count, label_pos) {
            emit(state, &format!("        brf     {target}"));
        } else {
            emit_long_brf(state, target);
        }
        return;
    }
    let line = format!("        brf     {target}");
    let out_offset = state.out.len();
    emit(state, &line);
    state.deferred_branches.push(DeferredBranch {
        out_offset,
        line,
        instruction_count: state.instruction_count - 1,
        target: target.to_string(),
        kind: BranchKind::Brf,
    });
}

fn emit_long_brt(state: &mut CodegenState, target: &str) {
    let skip = new_label(state);
    emit(state, &format!("        brf     {skip}"));
    emit(state, &format!("        la      r2,{target}"));
    emit(state, "        jmp     (r2)");
    emit(state, &format!("{skip}:"));
}

fn emit_long_brf(state: &mut CodegenState, target: &str) {
    let skip = new_label(state);
    emit(state, &format!("        brt     {skip}"));
    emit(state, &format!("        la      r2,{target}"));
    emit(state, "        jmp     (r2)");
    emit(state, &format!("{skip}:"));
}

/// Resolve deferred forward branches after a function is fully generated.
///
/// Checks each deferred short branch against the now-known label positions.
/// Out-of-range branches are expanded to long form by replacing the short
/// branch line in `state.out`. Processes from end to start so byte offsets
/// of earlier entries remain valid.
pub fn resolve_branches(state: &mut CodegenState) {
    // Process in reverse order so replacements don't shift earlier offsets
    let branches: Vec<DeferredBranch> = state.deferred_branches.drain(..).collect();
    for branch in branches.iter().rev() {
        let Some(&label_pos) = state.label_positions.get(&branch.target) else {
            // Label not found — leave as-is (will be a link error or it's in another function)
            continue;
        };
        if can_short_branch(branch.instruction_count, label_pos) {
            // Short branch is valid — no change needed
            continue;
        }
        // Need to expand to long form
        let short_line_with_newline = format!("{}\n", branch.line);
        let long_replacement = match branch.kind {
            BranchKind::Bra => {
                format!(
                    "        la      r2,{}\n        jmp     (r2)\n",
                    branch.target
                )
            }
            BranchKind::Brt => {
                let skip = new_label(state);
                format!(
                    "        brf     {skip}\n        la      r2,{}\n        jmp     (r2)\n{skip}:\n",
                    branch.target
                )
            }
            BranchKind::Brf => {
                let skip = new_label(state);
                format!(
                    "        brt     {skip}\n        la      r2,{}\n        jmp     (r2)\n{skip}:\n",
                    branch.target
                )
            }
        };
        let end = branch.out_offset + short_line_with_newline.len();
        state.out.replace_range(branch.out_offset..end, &long_replacement);
    }
}
