#![no_std]

extern crate alloc;
use alloc::string::String;
use core::marker::PhantomData;
use ecdysis::{EcdysisManager, CapabilityKey};

// Dummy println! macro for no_std environment
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => ({});
}

// OSTD: The OS Standard Library for Zenith
// Provides safe abstractions over raw syscalls.

/// A handle to a user-space object, ensuring kernel-level safety.
/// T must be Sized to ensure we know the memory layout.
pub struct UserHandle<T: Sized> {
    ptr: *mut T,
    _marker: PhantomData<T>,
}

impl<T: Sized> UserHandle<T> {
    /// Unsafe because the caller must ensure the pointer is valid.
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    pub fn as_ref(&self) -> &T {
        unsafe { &*self.ptr }
    }

    pub fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}

pub struct SyscallInterface {
    ecdysis: EcdysisManager,
}

impl SyscallInterface {
    pub fn new(ecdysis: EcdysisManager) -> Self {
        Self { ecdysis }
    }

    /// Safe syscall wrapper: send_ipc
    pub fn send_ipc(&self, _dest_pid: u64, _payload: &[u8], cap: &CapabilityKey) -> Result<(), String> {
        // 1. Verify capability
        if !self.ecdysis.verify(&cap.resource, "ipc_send") {
            return Err(String::from("Access Denied: Missing Capability"));
        }

        // 2. Perform mocked syscall
        // println!("[OSTD] Syscall: send_ipc to PID {}", dest_pid);
        Ok(())
    }

    /// Safe syscall wrapper: allocate_page (Mocked)
    pub fn allocate_page<T: Sized>(&self) -> Result<UserHandle<T>, String> {
        // In a real OS, this would trap to kernel to map a page.
        // Here we just mock a pointer (DANGEROUS in real life, okay for simulation structure)
        let layout = alloc::alloc::Layout::new::<T>();
        unsafe {
            let ptr = alloc::alloc::alloc(layout) as *mut T;
             Ok(UserHandle::from_raw(ptr))
        }
    }
}
