# Urgent Issues

Fix these before starting the next saga step.

## GH #19 — Negative integer division returns 0

Blocks clean implementation of residue (modulo) and any future ops that
divide negative numbers. Currently worked around in `int_residue()` by
converting to absolute values first.

## GH #20 — If/else chain stores wrong value in array

Blocks clean implementation of scan with reserved-word operators
(`ceil\`, `floor\`, `and\`, `or\`). Currently worked around by encoding
`RES_*` ids as negative numbers and decoding inline in the evaluator.

---

Neither is a hard blocker for forward progress — the workarounds function
correctly. But they add code complexity and could mask further issues in
future steps if the root causes aren't fixed.
