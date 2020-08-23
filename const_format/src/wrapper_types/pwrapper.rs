use crate::{
    formatting::{FormattingFlags, FormattingMode, StartAndArray, FOR_ESCAPING},
    pargument::Integer,
};

#[cfg(feature = "with_fmt")]
use crate::fmt::FormattingLength;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone)]
pub struct PWrapper<T>(pub T);

impl<'a, T> PWrapper<&'a [T]> {
    /// For constructing from a reference to an array.
    ///
    /// With slices you can do `PWrapper(slice)` instead.
    #[inline(always)]
    pub const fn slice(x: &'a [T]) -> Self {
        Self { 0: x }
    }
}

macro_rules! compute_hex_count {
    ($bits:expr, $int:expr, $with_0x:expr) => {{
        let with_0x = ($with_0x as usize) << 1;
        let i = ($bits - $int.leading_zeros()) as usize;
        (if i == 0 {
            1
        } else {
            (i >> 2) + ((i & 3) != 0) as usize
        }) + with_0x
    }};
}
macro_rules! compute_binary_count {
    ($bits:expr, $int:expr, $with_0b:expr) => {{
        let with_0b = ($with_0b as usize) << 1;
        let i = ($bits - $int.leading_zeros()) as usize;
        (if i == 0 { 1 } else { i }) + with_0b
    }};
}

macro_rules! impl_number_of_digits {
    (num number_of_digits;delegate $n:ident $len:ident)=>{
        $n.number_of_digits()
    };
    (num number_of_digits;128 $n:ident $len:ident)=>{{
        if $n >= 1_0000_0000_0000_0000{$n /= 1_0000_0000_0000_0000; $len += 16;}
        impl_number_of_digits!(num number_of_digits;64 $n $len)
    }};
    (num number_of_digits;64 $n:ident $len:ident)=>{{
        if $n >= 1_0000_0000_0000{$n /= 1_0000_0000_0000; $len += 12;}
        impl_number_of_digits!(num number_of_digits;32 $n $len)
    }};
    (num number_of_digits;32 $n:ident $len:ident)=>{{
        if $n >= 1_0000_0000{$n /= 100_000_000; $len += 8;}
        impl_number_of_digits!(num number_of_digits;16 $n $len)
    }};
    (num number_of_digits;16 $n:ident $len:ident)=>{{
        if $n >= 1_0000{$n /= 1_0000; $len += 4;}
        impl_number_of_digits!(num number_of_digits;8 $n $len)
    }};
    (num number_of_digits;8 $n:ident $len:ident)=>{{
        if $n >= 100{$n /= 100; $len += 2;}
        if $n >= 10{            $len += 1;}
        $len
    }};
    (@shared $This:ty, $bits:tt)=>{
        #[cfg(feature = "with_fmt")]
        impl PWrapper<$This> {
            pub const fn const_display_len(&self, f: &mut FormattingLength) {
                let len = self.compute_display_len(f.flags());
                f.add_len(len);
            }

            #[inline(always)]
            pub const fn const_debug_len(&self, f: &mut FormattingLength) {
                let len = self.compute_debug_len(f.flags());
                f.add_len(len);
            }
        }

        impl PWrapper<$This> {
            #[allow(unused_mut,unused_variables)]
            #[doc(hidden)]
            pub const fn compute_debug_len(self, fmt: FormattingFlags)-> usize {
                match fmt.mode() {
                    FormattingMode::Regular=>
                        self.compute_display_len(fmt),
                    FormattingMode::Hexadecimal=>
                        compute_hex_count!($bits, self.0, fmt.is_alternate()),
                    FormattingMode::Binary=>
                        compute_binary_count!($bits, self.0, fmt.is_alternate()),
                }
            }

            pub const fn hexadecimal_len(self, fmt: FormattingFlags)-> usize {
                compute_hex_count!($bits, self.0, fmt.is_alternate())
            }

            pub const fn binary_len(self, fmt: FormattingFlags)-> usize {
                compute_binary_count!($bits, self.0, fmt.is_alternate())
            }
        }
    };
    (impl_either;
        signed,
        ($This:ty, $Unsigned:ty),
        $bits:tt $(,)?
    )=>{
        impl_number_of_digits!{@shared $This, $bits}

        impl PWrapper<$This> {
            pub const fn unsigned_abs(self) -> $Unsigned {
                self.0.wrapping_abs() as $Unsigned
            }

            #[allow(unused_mut,unused_variables)]
            #[doc(hidden)]
            pub const fn compute_display_len(self, _: FormattingFlags)-> usize {
                let mut n = self.0.wrapping_abs() as $Unsigned;
                let mut len = 1 + (self.0 < 0) as usize;
                impl_number_of_digits!(num number_of_digits;$bits n len)
            }


        }
    };
    (impl_either;
        unsigned,
        ($This:ty, $Unsigned:ty),
        $bits:tt $(,)?
    )=>{
        impl_number_of_digits!{@shared $This, $bits}

        impl PWrapper<$This> {
            pub const fn unsigned_abs(self) -> $Unsigned {
                self.0
            }

            #[doc(hidden)]
            pub const fn compute_display_len(self, _: FormattingFlags)-> usize {
                let mut n = self.0;
                let mut len = 1usize;
                impl_number_of_digits!(num number_of_digits;$bits n len)
            }
        }
    };
}

impl_number_of_digits! {impl_either; signed  , (i8, u8), 8}
impl_number_of_digits! {impl_either; signed  , (i16, u16), 16}
impl_number_of_digits! {impl_either; signed  , (i32, u32), 32}
impl_number_of_digits! {impl_either; signed  , (i64, u64), 64}
impl_number_of_digits! {impl_either; signed  , (i128, u128), 128}
impl_number_of_digits! {impl_either; unsigned, (u8, u8), 8}
impl_number_of_digits! {impl_either; unsigned, (u16, u16), 16}
impl_number_of_digits! {impl_either; unsigned, (u32, u32), 32}
impl_number_of_digits! {impl_either; unsigned, (u64, u64), 64}
impl_number_of_digits! {impl_either; unsigned, (u128, u128), 128}

#[cfg(target_pointer_width = "16")]
type UWord = u16;
#[cfg(target_pointer_width = "32")]
type UWord = u32;
#[cfg(target_pointer_width = "64")]
type UWord = u64;
#[cfg(target_pointer_width = "128")]
type UWord = u128;

#[cfg(target_pointer_width = "16")]
type IWord = i16;
#[cfg(target_pointer_width = "32")]
type IWord = i32;
#[cfg(target_pointer_width = "64")]
type IWord = i64;
#[cfg(target_pointer_width = "128")]
type IWord = i128;

macro_rules! impl_for_xsize {
    ($XSize:ident, $XWord:ident) => {
        #[cfg(feature = "with_fmt")]
        impl PWrapper<$XSize> {
            pub const fn const_display_len(&self, f: &mut FormattingLength) {
                let len = self.compute_display_len(f.flags());
                f.add_len(len);
            }

            #[inline(always)]
            pub const fn const_debug_len(&self, f: &mut FormattingLength) {
                let len = self.compute_debug_len(f.flags());
                f.add_len(len);
            }
        }

        impl PWrapper<$XSize> {
            #[inline(always)]
            #[doc(hidden)]
            pub const fn compute_display_len(self, fmt: FormattingFlags) -> usize {
                PWrapper(self.0 as $XWord).compute_display_len(fmt)
            }

            #[inline(always)]
            #[doc(hidden)]
            pub const fn compute_debug_len(self, fmt: FormattingFlags) -> usize {
                PWrapper(self.0 as $XWord).compute_debug_len(fmt)
            }

            #[inline(always)]
            pub const fn hexadecimal_len(self, fmt: FormattingFlags) -> usize {
                PWrapper(self.0 as $XWord).hexadecimal_len(fmt)
            }

            #[inline(always)]
            pub const fn binary_len(self, fmt: FormattingFlags) -> usize {
                PWrapper(self.0 as $XWord).binary_len(fmt)
            }
        }
    };
}

impl_for_xsize! {usize, UWord}
impl_for_xsize! {isize, IWord}

impl PWrapper<usize> {
    pub const fn unsigned_abs(self) -> usize {
        self.0
    }
}

impl PWrapper<isize> {
    pub const fn unsigned_abs(self) -> usize {
        self.0.wrapping_abs() as usize
    }
}

impl Integer {
    #[inline]
    const fn as_negative(self) -> i128 {
        (self.unsigned as i128).wrapping_neg()
    }
}

impl PWrapper<Integer> {
    pub const fn to_start_array_binary(self, flags: FormattingFlags) -> StartAndArray<[u8; 130]> {
        let mut n = if self.0.is_negative {
            self.0.as_negative() as u128
        } else {
            self.0.unsigned
        };

        n &= *self.0.mask;

        let mut out = StartAndArray {
            start: 130,
            array: [0u8; 130],
        };

        loop {
            out.start -= 1;
            let digit = (n & 1) as u8;
            out.array[out.start] = b'0' + digit;
            n = n >> 1;
            if n == 0 {
                break;
            }
        }

        if flags.is_alternate() {
            out.start -= 1;
            out.array[out.start] = b'b';
            out.start -= 1;
            out.array[out.start] = b'0';
        }

        out
    }

    pub const fn to_start_array_hexadecimal(
        self,
        flags: FormattingFlags,
    ) -> StartAndArray<[u8; 34]> {
        let mut n = if self.0.is_negative {
            self.0.as_negative() as u128
        } else {
            self.0.unsigned
        };

        n &= *self.0.mask;

        let mut out = StartAndArray {
            start: 34,
            array: [0u8; 34],
        };

        loop {
            out.start -= 1;
            let digit = (n & 0xF) as u8;
            out.array[out.start] = match digit {
                0..=9 => b'0' + digit,
                _ => b'A' - 10 + digit,
            };
            n = n >> 4;
            if n == 0 {
                break;
            }
        }

        if flags.is_alternate() {
            out.start -= 1;
            out.array[out.start] = b'x';
            out.start -= 1;
            out.array[out.start] = b'0';
        }

        out
    }

    pub const fn to_start_array_display(self) -> StartAndArray<[u8; 40]> {
        let mut out = StartAndArray {
            start: 40,
            array: [0u8; 40],
        };

        let mut n = self.0.unsigned;

        loop {
            out.start -= 1;
            let digit = (n % 10) as u8;
            out.array[out.start] = b'0' + digit;
            n /= 10;
            if n == 0 {
                break;
            }
        }

        if self.0.is_negative {
            out.start -= 1;
            out.array[out.start] = b'-';
        }

        out
    }

    #[inline(always)]
    pub const fn to_start_array_debug(self) -> StartAndArray<[u8; 40]> {
        self.to_start_array_display()
    }
}

impl PWrapper<&[u8]> {
    pub const fn compute_utf8_debug_len(self) -> usize {
        let mut sum = self.0.len();
        __for_range! {i in 0..self.0.len() =>
            let c = self.0[i];
            if c < 128 {
                let shifted = 1 << c;
                if (FOR_ESCAPING.is_escaped & shifted) != 0 {
                    sum += if (FOR_ESCAPING.is_backslash_escaped & shifted) == 0 {
                        3 // `\x01` only add 3 characters
                    } else {
                        1 // Escaped with a backslash
                    };
                }

            }
        }
        sum + 2 // The quote characters
    }
}

impl PWrapper<&str> {
    #[inline(always)]
    #[doc(hidden)]
    pub const fn compute_debug_len(self, _: FormattingFlags) -> usize {
        PWrapper(self.0.as_bytes()).compute_utf8_debug_len()
    }

    #[inline(always)]
    #[doc(hidden)]
    pub const fn compute_display_len(self, _: FormattingFlags) -> usize {
        self.0.len()
    }
}