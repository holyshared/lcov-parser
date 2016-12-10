// Copyright (c) 2015-2016 lcov-parser developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[macro_export]
macro_rules! impl_summary {
    ($dest:ty, $field:ident<$key:ty, $value:ty>) => {
        impl $crate::report::summary::Summary<$key, $value> for $dest {
            fn iter(&self) -> ::std::collections::btree_map::Iter<$key, $value> {
                self.$field.iter()
            }
            fn contains_key(&self, key: &$key) -> bool {
                self.$field.contains_key(key)
            }
            fn get(&self, key: &$key) -> Option<&$value> {
                self.$field.get(key)
            }
            fn len(&self) -> usize {
                self.$field.len()
            }
        }
    }
}

#[macro_export]
macro_rules! impl_try_merge_self_summary {
    ($dest:ty:$field:ident, $err:ty) => {
        impl<'a> $crate::merger::ops::TryMerge<&'a $dest> for $dest {
            type Err = $err;

            fn try_merge(&mut self, other: &'a $dest) -> MergeResult<Self::Err> {
                for (key, other_value) in other.iter() {
                    if !self.$field.contains_key(key) {
                        self.$field.insert(key.clone(), other_value.clone());
                        continue;
                    }
                    let mut value = self.$field.get_mut(key).unwrap();
                    let _ = try!(value.try_merge(other_value));
                }
                Ok(())
            }
        }
    }
}
