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
