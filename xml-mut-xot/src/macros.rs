#[macro_export]
macro_rules! ok_or_return_false {
    ( $e:expr ) => {
        if let Some(n) = $e {
            n
        } else {
            return false;
        }
    };
}