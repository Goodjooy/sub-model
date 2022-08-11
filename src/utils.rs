macro_rules! darling_err {
    ($res:expr) => {
        match $res {
            Ok(data) => data,
            Err(err) => return err.write_errors().into(),
        }
    };
}
macro_rules! syn_err {
    ($res:expr) => {
        match $res {
            Ok(data) => data,
            Err(err) => return err.to_compile_error().into(),
        }
    };
}
