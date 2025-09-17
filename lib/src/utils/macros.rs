#[macro_export]
macro_rules! tern {
    ($cond:expr, $a:expr, $b:expr) => {
        if $cond {
            $a
        } else {
            $b
        }
    };
}

#[macro_export]
macro_rules! select {
    ($iter:expr; $first:expr $(, $rest:expr)*) => {
        {
            let mut __iter = &mut $iter;
            let __first_item = __iter.nth($first).unwrap();
            select!(@acc __iter, (__first_item), $first; $($rest),*)
        }
    };

    (@acc $iter:expr, ($($picked:expr),*), $prev:expr; $next:expr $(, $rest:expr)*) => {
        {
            let __skip = $next - $prev - 1;
            let __item = $iter.nth(__skip).unwrap();
            select!(@acc $iter, ($($picked,)* __item), $next; $($rest),*)
        }
    };

    (@acc $_iter:expr, ($($picked:expr),*), $_last:expr;) => {
        ($($picked),*)
    };
}

#[macro_export]
macro_rules! defer {
    ($value:expr; $task:expr) => {{
        let __res = $value;
        $task;
        __res
    }};
}

#[macro_export]
macro_rules! borrow_loop {
    ($iter:expr, $var:ident, $body:expr) => {{
        while let Some($var) = $iter.next() {
            $body
        }
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn tern() {
        assert_eq!(tern!(true, 1, 2), 1);
        assert_eq!(tern!(false, 1, 2), 2);
    }

    #[test]
    fn select() {
        let mut iter = [1, 2, 3, 4, 5].into_iter();
        assert_eq!(select!(iter; 1,3,4), (2, 4, 5));
    }

    #[test]
    fn defer() {
        let mut a = 4;
        let calculated = { defer!("some value"; a += 1) };
        assert_eq!(calculated, "some value");
        assert_eq!(a, 5);
    }
}
