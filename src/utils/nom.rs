// Macro to map function returning Cesr result to nom compatible
#[macro_export]
macro_rules! nomify {
    ($func:expr) => {
        |bytes: &'a [u8]| {
            $func(bytes).map_err(|_| {
                nom::Err::Error((bytes, nom::error::ErrorKind::IsNot))
            })
        }
    };
}
