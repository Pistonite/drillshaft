/// Stub macro for build script to generate binaries provided by a package
macro_rules! register_binaries {
    ($($l:literal),*) => {};
}
pub(crate) use register_binaries;

/// Convenience macro to generate implementation for binary_dependencies
macro_rules! binary_dependencies {
    ($($ident:ident),* $(,)?) => {
        pub fn binary_dependencies() -> EnumSet<BinId> { enum_set! { $(BinId::$ident)|* } }
    };
}
pub(crate) use binary_dependencies;

/// Convenience macro to generate implementation for config_dependencies
macro_rules! config_dependencies {
    ($($ident:ident),* $(,)?) => {
        pub fn config_dependencies() -> EnumSet<PkgId> { enum_set! { $(PkgId::$ident)|* } }
    };
}
pub(crate) use config_dependencies;

/// Generate a static VERSION_CACHE constant
macro_rules! version_cache {
    (pub static $ident:ident = $expr:expr) => {
        pub static $ident: VersionCache = VersionCache::new(stringify!($expr), $expr);
    };
    (static $ident:ident = $expr:expr) => {
        static $ident: VersionCache = VersionCache::new(stringify!($expr), $expr);
    };
}
pub(crate) use version_cache;
