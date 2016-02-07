// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

macro_rules! assert_true {
    ( $ cond : expr ) => (
        assert!($cond)
    );
    ($ cond : expr , $ ( $ arg : tt )+ ) => (
        assert!($cond, $($arg)+)
    );
}

macro_rules! assert_false {
    ( $ cond : expr ) => (
        assert_eq!($cond, false)
    );
    ($ cond : expr , $ ( $ arg : tt )+ ) => (
        assert_eq!($cond, false, $($arg)+)
    );
}
