# epbf-vm

Writing an eBPF VM in rust to learn about rust, VMs, and eBPF

## eBPF
notes taken from [wiki pag](wiki-ebpf)

### Virtual Machine
takes in eBPF bytecode instructions then converts them to native machine instructions
Just-in-Time compiles the code

eleven 64-bit registers with 32-bit subregisters
program counter
512-byte BPF stack space

### Calling Conventions
#### Tail calls
used to call and execute another eBPF program and replace the execution context
basically allowing an eBPF program to call another eBPF program

implemented as long jumps reusing the same stack frame

useful in eBPF because of the limited stack space of 512 bytes. Common use case
for tail calls is breaking up complexity of eBPF program over several smaller
programs

Other use case is replacing/extending logic by changing the contents of the
program array while still in use. Example is updating a program version without
downtime or turning logic on/off.

### Verifier
very important component of eBPF with responsibility of ensuring program is safe to execute
through static analysis.

assesses all possible execution paths by stepping through instructions in order
and evaluating them. Verification starts with depth-first search through all
possible paths.

while stepping through instruction, tracking state of registers and stack. If
at any point instruction could lead to unsafe state, verification fails.

Which rules are checked may depend on the type of program, possible rules include:
* always terminating in a reasonable amount of time (no infinite loops or infinite recursion)
* checking for ability to read arbitrary memory (could lead to leaking of sensitive info)
* checking for accessing memory outside of packet bounds (adjacent memory could be sensitive)
* checking that program doesn't allow deadlocking
* checking for reading uninitialized memory
* ...

[wiki-ebpf]: https://en.wikipedia.org/wiki/EBPF
