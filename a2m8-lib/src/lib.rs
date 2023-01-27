pub(crate) mod assets;
pub(crate) mod modules;
pub(crate) mod private;

pub use modules::require;

pub(crate) mod prelude {
    pub use tealr::mlu::TypedFunction;
    pub use tealr::mlu::UserDataWrapper;
    pub mod mlua {
        pub use tealr::mlu::mlua::*;
    }
    pub use tealr::mlu::generics::*;
}
