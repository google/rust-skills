// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::mem::MaybeUninit;

/// Writes 42 into `out_index` and returns 0 on success, or -1 if `out_index` is null.
///
/// `out_index` is an out-parameter that may point to uninitialized C caller
/// memory and is only written to, never read before writing.
#[no_mangle]
pub extern "C" fn ffi_write_index(out_index: Option<&mut usize>) -> i32 {
    let Some(out_index) = out_index else {
        return -1;
    };
    *out_index = 42;
    0
}

pub fn init_u32_via_uninit_ref() -> u32 {
    let mut slot = MaybeUninit::<u32>::uninit();
    // SAFETY: `slot.as_mut_ptr()` is non-null, aligned, and points to `slot`.
    let r: &mut u32 = unsafe { &mut *slot.as_mut_ptr() };
    *r = 7;
    // SAFETY: `*r = 7` initialized `slot` above.
    unsafe { slot.assume_init() }
}

pub fn overwrite_uninit_string() -> String {
    let mut slot = MaybeUninit::<String>::uninit();
    // SAFETY: `slot.as_mut_ptr()` is non-null, aligned, and points to `slot`
    // (UCG#414).
    let r: &mut String = unsafe { &mut *slot.as_mut_ptr() };
    *r = String::from("hello");
    // SAFETY: `*r` was written above.
    unsafe { slot.assume_init() }
}
