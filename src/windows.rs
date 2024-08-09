use windows::Win32::System::{
    Diagnostics::Debug::FlushInstructionCache, Threading::GetCurrentProcess,
};

pub(crate) unsafe fn os_arch_clear_cache<T>(start: *const T, end: *const T) -> bool {
    let res = unsafe {
        FlushInstructionCache(
            GetCurrentProcess(),
            Some(start.cast()),
            (end as *const u8).offset_from(start as *const u8) as usize,
        )
    };
    res.is_ok()
}
