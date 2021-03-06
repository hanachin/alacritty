// Copyright 2016 Joe Wilm, The Alacritty Project Contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use foreign_types::{ForeignTypeRef};

use super::ffi::{FcCharSet, FcCharSetDestroy, FcCharSetAddChar};
use super::ffi::{FcCharSetCreate};

foreign_type! {
    type CType = FcCharSet;
    fn drop = FcCharSetDestroy;
    pub struct CharSet;
    pub struct CharSetRef;
}

impl CharSet {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for CharSet {
    fn default() -> Self {
        CharSet(unsafe { FcCharSetCreate() })
    }
}

impl CharSetRef {
    pub fn add(&mut self, glyph: char) -> bool {
        unsafe {
            FcCharSetAddChar(
                self.as_ptr(),
                glyph as _
            ) == 1
        }
    }
}

