# lint6502

Linter and beautifier for asm code written for the ca65 compiler (hosted here https://github.com/cc65/cc65).

The intention is to support these:
  - Homogeinize indentation/formatting
  - Lint identifier names such that they are consistent (constants, addresses, labels)
  - Annotate routines with respect to the memory footprint
  - Identify possible issues with memory share

Optionally these:
  - Produce plain ca65 code for more complex input (using this as a library to extend the ca65 language)
  - Rewrite my tools that produce ca65 code string-wise
