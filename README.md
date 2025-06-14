# clear-cache

A native implementation of `__builtin___clear_cache` without dependency of GCC/Clang.

From [GCC documentation](https://gcc.gnu.org/onlinedocs/gcc/Other-Builtins.html#index-_005f_005fbuiltin_005f_005f_005fclear_005fcache):

> This function is used to flush the processor’s instruction cache for the region of memory between begin inclusive and end exclusive. Some targets require that the instruction cache be flushed, after modifying memory containing code, in order to obtain deterministic behavior.
>
> If the target does not require instruction cache flushes, `__builtin___clear_cache` has no effect. Otherwise either instructions are emitted in-line to clear the instruction cache or a call to the `__clear_cache` function in libgcc is made.

From [LLVM documentation](https://llvm.org/docs/LangRef.html#llvm-clear-cache-intrinsic):

> The `llvm.clear_cache` intrinsic ensures visibility of modifications in the specified range to the execution unit of the processor. On targets with non-unified instruction and data cache, the implementation flushes the instruction cache.
>
> On platforms with coherent instruction and data caches (e.g. x86), this intrinsic is a nop. On platforms with non-coherent instruction and data cache (e.g. ARM, MIPS), the intrinsic is lowered either to appropriate instructions or a system call, if cache flushing requires special privileges.
>
> The default behavior is to emit a call to `__clear_cache` from the run time library.
>
> This intrinsic does *not* empty the instruction pipeline. Modifications of the current function are outside the scope of the intrinsic.

Current implementation is taken from [LLVM's implementation](https://github.com/llvm/llvm-project/blob/main/compiler-rt/lib/builtins/clear_cache.c)

Current CI-tested platforms:

* Linux

    * `x86_64-unknown-linux-gnu`
    * `x86_64-unknown-linux-musl`
    * `i686-unknown-linux-gnu`
    * `aarch64-unknown-linux-gnu`
    * `riscv64gc-unknown-linux-gnu`
    * `loongarch64-unknown-linux-gnu`
    * `aarch64-linux-android`
* macOS

    * `aarch64-apple-darwin`
* Windows

    * `x86_64-pc-windows-msvc`
    * `i686-pc-windows-msvc`

Note that it seems that `clear-cache` works in `armv7-linux-androideabi`, but due to a bug in QEMU's arm code (see [#2](https://github.com/Evian-Zhang/clear-cache/issues/2)), we cannot test it in CI.
