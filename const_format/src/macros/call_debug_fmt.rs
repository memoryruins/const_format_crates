/// For debug formatting of some specific generic std types, and other types.
///
/// # Errors
///
/// This macro calls the appropriate formatting methods, and `return`s errors when they happen.
///
/// # Macro variants
///
/// The macro has these variants:
///
/// - `slice` (also `array`): to format a slice or an array of *any debug type.
///
/// - `Option`: to format an `Option` of *any debug type.
///
/// - `newtype`: to format a single field tuple struct (eg: `struct Foo(Bar);`)
/// which wraps *any debug type.
///
/// - `std`: to format the standard library types where `PWrapper<ThatType>`
/// has a `const_debug_fmt` method.
///
/// - `other`: to format non-standard-library types that have a `const_debug_fmt` method.
///
/// *"any debug type" meaning types that have a `const_debug_fmt` method
///
/// # Example
///
/// Printing all of the kinds of types this supports.
///
/// ```rust
/// #![feature(const_mut_refs)]
///
/// use const_format::{
///     for_examples::{Point3, Unit},
///     Error, Formatter, FormattingFlags, StrWriter,
///     call_debug_fmt, strwriter_as_str, try_, unwrap,
/// };
///
/// use std::num::Wrapping;
///
/// const CAP: usize = 512;
///
/// // `call_debug_fmt` implicitly returns on error,
/// // so the function has to return a `Result<_, const_format::Error>`
/// const fn make() -> Result<StrWriter<[u8; CAP]>, Error> {
///     let mut writer = StrWriter::new([0; CAP]);
///     let mut fmt = Formatter::from_sw(&mut writer, FormattingFlags::NEW);
///     let mut fmt = fmt.debug_struct("Foo");
///
///     let point = Point3{ x: 5, y: 8, z: 13 };
///
///     call_debug_fmt!(array, [Unit, Unit], fmt.field("array") );
///     call_debug_fmt!(slice, [0u8, 1], fmt.field("slice") );
///     call_debug_fmt!(Option, Some(point), fmt.field("option") );
///     call_debug_fmt!(newtype NumWrapping, Wrapping(255u16), fmt.field("newtype") );
///     call_debug_fmt!(std, false, fmt.field("std_") );
///     call_debug_fmt!(other, point, fmt.field("other") );
///
///     try_!(fmt.finish());
///     Ok(writer)
/// }
///
/// const TEXT: &str = {
///     let promoted = &make();
///     strwriter_as_str!(unwrap!(promoted))
/// };
///
/// const EXPECTED: &str = "\
///     Foo { \
///         array: [Unit, Unit], \
///         slice: [0, 1], \
///         option: Some(Point3 { x: 5, y: 8, z: 13 }), \
///         newtype: NumWrapping(255), \
///         std_: false, \
///         other: Point3 { x: 5, y: 8, z: 13 } \
///     }\
/// ";
///
/// assert_eq!(TEXT, EXPECTED);
///
///
/// ```
#[macro_export]
macro_rules! call_debug_fmt {
    (array, $expr:expr, $formatter:expr $(,)* ) => {{
        match (&$expr, $formatter.borrow_mutably()) {
            (expr, formatter) => {
                let mut n = 0;
                let len = expr.len();
                let mut f = formatter.debug_list();
                while n != len {
                    $crate::__call_debug_fmt_dispatch!(&expr[n], f.entry());
                    n += 1;
                }
                $crate::try_!(f.finish());
            }
        }
    }};
    (slice, $expr:expr, $formatter:expr $(,)*) => {
        $crate::call_debug_fmt!(array, $expr, $formatter)
    };
    (Option, $expr:expr, $formatter:expr $(,)*) => {{
        match $formatter.borrow_mutably() {
            formatter => $crate::try_!(match &$expr {
                $crate::pmr::Some(x) => {
                    let mut f = formatter.debug_tuple("Some");
                    $crate::__call_debug_fmt_dispatch!(x, f.field());
                    f.finish()
                }
                $crate::pmr::None => formatter.write_str("None"),
            }),
        }
    }};
    (newtype $name:ident, $expr:expr, $formatter:expr $(,)*) => {
        match (&$expr, $formatter.borrow_mutably()) {
            (newtype_, formatter) => {
                let mut f = formatter.debug_tuple(stringify!($name));
                $crate::__call_debug_fmt_dispatch!(&newtype_.0, f.field());
                $crate::try_!(f.finish());
            }
        }
    };
    (std, $expr:expr, $formatter:expr $(,)*) => {
        if let Err(e) = $crate::coerce_to_fmt!(&$expr).const_debug_fmt($formatter) {
            return Err(e);
        }
    };
    (other, $expr:expr, $formatter:expr $(,)*) => {
        if let Err(e) = $expr.const_debug_fmt($formatter) {
            return Err(e);
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __call_debug_fmt_dispatch {
    ($e:expr, $f:expr) => {
        if let Err(e) = $crate::coerce_to_fmt!(&$e).const_debug_fmt($f) {
            return Err(e);
        }
    };
}
