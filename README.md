## Lust = **L**ogic + R**ust**

This a solid base for a future general purpose logic toolkit. 

Kinda just hacked this together in 3hrs hrs on a cold saturday evening to improve our rust skillz

### What we've done so far

So far we can tokenise and parse logical expressions into an Expression Tree. This tree may contain variables and literals.
An expression can be evaluated by providing a HashMap of variable assignments for the free variables in the expression.

### The future (oooooh ?!?!?)

Some ideas we've had

- REPL to make assignments and evaluate logical expressions
- Automatically Generate Semantic Tableux Proofs and or Natural Deduction Proofs of inputs.
- more advanced parser to allow logical modelling and the creation of functions. 
- Visualize logical expressions somehow??

This could be usefuil to future logic students (comp1600 | comp2620 etc) as a helper or utility tool.
