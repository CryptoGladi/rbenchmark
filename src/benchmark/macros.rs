macro_rules! vec_box {
    ($($x:expr),*) => {
        vec![$(Box::new($x),)*]
    };
}

pub(crate) use vec_box;
