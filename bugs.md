# Parser bugs

- [x] Pratt loop checks precedence against `curr_token` instead of the lookahead token, which breaks operator precedence and chaining.
- [x] Infix operator parselet passes precedence for the right-hand token instead of the operator, causing `+`/`*` binding errors.
- [x] Grouped expression parselet does not consume the closing `)` token, so grouped expressions leak tokens.
- [x] Function call argument parsing never advances after parsing an argument and does not consume the closing `)`, causing infinite loops.
- [x] Array access parselet does not consume the closing `]`, which can leak tokens inside larger expressions (e.g., array access as a function arg).
- [x] Object access parselet allowed function-call binding on the right side (`obj.method(arg)` parsed as `obj.(method(arg))`).

# Incomplete implementations
- [x] Complete function statement lexing and parsing
- [x] Complete return statement lexing and parsing
- [ ] The object system for handling variables, etc
