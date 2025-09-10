mod bindings;
pub use bindings::*;

macro_rules! __link {
    ($library:literal $abi:literal $($link_name:literal)? fn $($function:tt)*) => (
        extern $abi {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    )
}

pub(crate) use __link as link;
