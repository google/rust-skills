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

/// Read a value from a raw pointer.
///
/// # Safety
///
/// The caller must ensure that:
/// 1. `ptr` is non-null and properly aligned for `i32`.
/// 2. `ptr` is valid for reads of `size_of::<i32>()` bytes for the duration of this call.
/// 3. The entire 4-byte range lies within a single live allocation.
/// 4. The memory points to a valid, initialized `i32` value.
/// 5. The memory is not mutated by any other pointer or reference for the duration
///    of this call, except as permitted by `UnsafeCell`.
pub unsafe fn read_value(ptr: *const i32) -> i32 {
    // Safety
    // Operation: dereference `ptr` to read an `i32`.
    // Required contract: `ptr` must be non-null, aligned, initialized, and valid
    // for reads of 4 bytes within one allocation, with no concurrent mutation.
    // Evidence:
    // - By this function's `# Safety` preconditions (1-5), the caller guarantees
    //   alignment, non-nullness, initialization, single-allocation validity,
    //   and the absence of data races for the duration of the call.
    // - AXIOM: `i32` is `Copy`, so dereferencing `*const i32` performs a bitwise
    //   read (similar to `ptr::read`) and does not move or drop the original value.
    // Therefore, the unsafe dereference is soundly discharged.
    unsafe { *ptr }
}
