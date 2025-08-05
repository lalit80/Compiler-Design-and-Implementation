# Compiler-Design-and-Implementation


Designed and implemented a compiler backend for a custom programming language over a six-month period.

Developed a syntax verifier and parser for the custom language, adhering to the grammar rules defined in 
rules.txt.

Engineered a variable-to-register allocation pass using a graph-coloring algorithm to optimize code and minimize memory access.

Implemented a Def-Use and In-Out analysis to manage variable lifetimes and dependencies, improving the efficiency of the register allocation process.

Created a code translator that converted the parsed language into assembly-like instructions, including 
LDR (Load Register), STR (Store Register), and BL (Branch with Link).

Utilized helper scripts, like 
script.sed, for automated code preprocessing during the development cycle.
