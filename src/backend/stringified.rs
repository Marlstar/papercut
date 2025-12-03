#[macro_export]
macro_rules! backends {
    [$($backend:ident $str:expr,)+] => {
        impl std::fmt::Display for Backend {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let s = match self {
                    $(Self::$backend => $str,)+
                };
                f.write_str(s)
            }
        }

        impl TryFrom<&str> for Backend {
            type Error = String;
            fn try_from(s: &str) -> Result<Self, Self::Error> {
                match s {
                    $($str => Ok(Self::$backend),)+
                    _ => Err(format!("Invalid backend '{s}'")),
                }
            }
        }
    }
}
pub(super) use backends;
