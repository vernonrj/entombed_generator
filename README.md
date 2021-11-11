Entombed Generator

Uses the rules derived in the paper [Explaining the Entombed Algorithm](https://arxiv.org/abs/2104.09982) on arxiv.

The paper explains the algorithm, and the rules derived from the lookup table.

Where `0` is Path and `1` is Wall:

> 1. No 2 ×2 squares of the same type are allowed
> 2. No wall or path is allowed to start or end with thickness one
> 3. Every path in any given line must be connected to a path in the next line
> 
> - Invariant 1 gives:
>   - 0 0 0 `*` → 1
>   - 1 1 1 `*` → 0
> - Invariant 2 gives:
>   - `*` `*` 0 1 0 → 1
>   - `*` `*` 1 0 1 → 0
>   - 0 1 0 `*` `*` → 1
>   - 1 0 1 `*` `*` → 0
> - Invariant 3 gives:
>   - `*` 1 0 0 1 → 0
>   - `*` `*` 1 0 1 → 0 (already a rule)