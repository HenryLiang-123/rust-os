# rust-os
Minimal OS in pure Rust

## Minimal Rust Kernel

1. Since our kernel is designed to run on bare metal, we want to define some configurations for our target system. We can reuse most of the configuration parameters from where linux is the target, but we need to set the OS to none.
2. Instead of using the platform's default linker, we use a cross platform LLD linker that is shipped with Rust to link our kernel.
3. The target does not support stack unwinding on panic. Stack unwinding is useful in more high-level programming to ensure proper resource cleanup and prevent memory leaks. In this Rust kernel, we use `no_std`, so we should simply abort when we encounter an error. This also makes the process faster and more minimal. Moreover, kernel-level errors are usually fatal.
4. We also want to `disable-redzone`. The redzone is a 128-byte area below the stack pointer to be used by functions that don't call other functions. In kernel development, the stack space below the stack pointer might not always be safe, so interrupt handlers may corrupt the data. This is because interrupt handlers do not use the same stack as normal execution.
5. 
