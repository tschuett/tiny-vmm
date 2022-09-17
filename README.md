# tiny-vmm
Tiny VMM


```rust
// Dummy x86 code that just calls cpuid and halt.
let x86_code = [0x0F, 0xA2 /*cpuid*/, 0xf4 /* hlt */];
```
