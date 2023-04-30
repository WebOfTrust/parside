// Helper macros to map function returning parside result to nom compatible
#[macro_export]
macro_rules! nomify {
    ($func:expr) => {
        |bytes: &'a [u8]| {
            $func(bytes).map_err(|_| {
                nom::Err::Error(nom::error::Error::new(bytes, nom::error::ErrorKind::IsNot))
            })
        }
    };
}
