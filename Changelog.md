This is the changelog,summarising changes in each version(some minor changes may be ommited).

# 0.2 

Every single new item added requires Rust nightly to use, with at least the "fmt" cargo feature enabled.

Defined a `core::fmt`-like API with these these types:
`ComputeStrLength`, `DebugList`, `DebugSet`, `DebugStruct`, `DebugTuple`, `Error`, `Formatter`, `FormattingFlags`, `NumberFormatting`, `StrWriter`, `StrWriterMut`, `NoEncoding`, `Utf8Encoding`.

Added `formatc` macro, for formatting std and user-defined types into a `&'static str` constant.

Added `writec` macro, for writing formatted std and user-defined types, 
into a type that implements `WriteMarker`.

Added `marker_traits::FormatMarker` trait, for types that implement const formatting,
with either the `const_debug_fmt`, or `const_display_fmt` inherent methods.

Added `ConstDebug` derive macro, for implementing `FormatMarker`,
and implement the `const_debug_fmt` inherent method.

Added `marker_traits::WriteMarker` trait, for types that can be written into,
defining the `borrow_mutably` and `make_formatter` methods.

Added these type in `marker_traits` module: `IsAFormatMarker`, `IsAStrWriter`, `IsAWriteMarker`, 
`IsArrayKind`, `IsNotAStrWriter`, `IsNotStdKind`, `IsStdKind`

Added hexadecimal and binary formatting to the `formatcp` macro
(also usable in `formatc`, and `writec`)

Defined the `AsciiStr` type, a wrapper type for `&[u8]` slices which are valid ascii,
with an `ascii_str` macro for constructing it at compile-time,
and `wrapper_types::NotAsciiError` returned by the fallible constructor.

Exposed the `PWrapper` type, wrapper for std types to call some methods on them.

Defined the `Sliced` type, to output a slice from a `&str`.

Defined these macros for implementing/doing compile-time formatting:
`call_debug_fmt`, `coerce_to_fmt`, `impl_fmt`

Defined the `strwriter_as_str` macro to cast a `&'static StrWriter` to a `&'static str`

Defined these error handling macros: `try_`, `unwrap`, `unwrap_or_else`

Defined the `for_examples` module with examples of types that implement const formatting.

Defined these utility functions in the `utils` module: 
`slice_up_to_len`, `slice_up_to_len_alt`, `str_eq u8`, `slice_eq `

Fixed error reporting in `formatcp` and `concatcp` macros,
now compiler errors point at the argument that caused an error rather than the whole macro invocation.

Added the "fmt" cargo feature, to enable the `fmt`-like API, and every other thing that depends on it.

Added the "derive" cargo feature, to enable the `ConstDebug` macro.

Added the "constant_time_as_str", to optimize some methods, requires additional nightly features.

Made `syn` an optional dependency, only enabled when the "derive" feature is used.

Added `unicode-xid` dependency.

# 0.1

Created `const_format` crate,
`const_format_proc_macros` crate(implementation detail of `const_format`)

Defined the concatcp macro,
for concatenating constants of primitive types into a `&'static str` constant.

Defined the formatcp macro,
for formatting constants of primitive types into a `&'static str` constant.

Added dependencies: syn with none of the expensive features, quote, proc_macro2

