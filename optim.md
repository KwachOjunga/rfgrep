# Optim branch
___

The primary objectives of optim:
  - Lower the number of instructions produced by rfgrep 
  - Improve its highlighting capabilities and also reduce the errors in its output.
  - It also intends to trim the codebase to its relevant parts.
    - Anything that is not presently in use should go.
    - Modules that have no utility should disappeear.
  - Learn the structure of the cache that rfgrep uses. 

 [Note:] if you are reading this; this branch is yet to be released.


21/02/2026

-- current output errs on both after and before texts. Nothing is captured in before.
-- remove `src/output` directory.
