// Generated by `wit-bindgen` 0.41.0. DO NOT EDIT!
// Options used:
//   * runtime_path: "wit_bindgen_rt"
#[rustfmt::skip]
#[allow(dead_code, clippy::all)]
pub mod hyperlight_wasm_examples {
    pub mod calculator {
        #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
        pub mod add {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn add(x: u32, y: u32) -> u32 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "hyperlight-wasm-examples:calculator/add"
                    )]
                    unsafe extern "C" {
                        #[link_name = "add"]
                        fn wit_import0(_: i32, _: i32) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    unsafe extern "C" fn wit_import0(_: i32, _: i32) -> i32 {
                        unreachable!()
                    }
                    let ret = unsafe { wit_import0(_rt::as_i32(&x), _rt::as_i32(&y)) };
                    ret as u32
                }
            }
        }
        #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
        pub mod subtract {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn subtract(x: u32, y: u32) -> u32 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "hyperlight-wasm-examples:calculator/subtract"
                    )]
                    unsafe extern "C" {
                        #[link_name = "subtract"]
                        fn wit_import0(_: i32, _: i32) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    unsafe extern "C" fn wit_import0(_: i32, _: i32) -> i32 {
                        unreachable!()
                    }
                    let ret = unsafe { wit_import0(_rt::as_i32(&x), _rt::as_i32(&y)) };
                    ret as u32
                }
            }
        }
        #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
        pub mod multiply {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn multiply(x: u32, y: u32) -> u32 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "hyperlight-wasm-examples:calculator/multiply"
                    )]
                    unsafe extern "C" {
                        #[link_name = "multiply"]
                        fn wit_import0(_: i32, _: i32) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    unsafe extern "C" fn wit_import0(_: i32, _: i32) -> i32 {
                        unreachable!()
                    }
                    let ret = unsafe { wit_import0(_rt::as_i32(&x), _rt::as_i32(&y)) };
                    ret as u32
                }
            }
        }
    }
}
#[rustfmt::skip]
#[allow(dead_code, clippy::all)]
pub mod exports {
    pub mod hyperlight_wasm_examples {
        pub mod calculator {
            #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
            pub mod calculate {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[repr(u8)]
                #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
                pub enum Op {
                    Add,
                    Subtract,
                    Multiply,
                }
                impl ::core::fmt::Debug for Op {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            Op::Add => f.debug_tuple("Op::Add").finish(),
                            Op::Subtract => f.debug_tuple("Op::Subtract").finish(),
                            Op::Multiply => f.debug_tuple("Op::Multiply").finish(),
                        }
                    }
                }
                impl Op {
                    #[doc(hidden)]
                    pub unsafe fn _lift(val: u8) -> Op {
                        if !cfg!(debug_assertions) {
                            return ::core::mem::transmute(val);
                        }
                        match val {
                            0 => Op::Add,
                            1 => Op::Subtract,
                            2 => Op::Multiply,
                            _ => panic!("invalid enum discriminant"),
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_evalexpression_cabi<T: Guest>(
                    arg0: i32,
                    arg1: i32,
                    arg2: i32,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::evalexpression(
                        Op::_lift(arg0 as u8),
                        arg1 as u32,
                        arg2 as u32,
                    );
                    _rt::as_i32(result0)
                }
                pub trait Guest {
                    fn evalexpression(op: Op, x: u32, y: u32) -> u32;
                }
                #[doc(hidden)]
                macro_rules! __export_hyperlight_wasm_examples_calculator_calculate_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[unsafe (export_name =
                        "hyperlight-wasm-examples:calculator/calculate#evalexpression")]
                        unsafe extern "C" fn export_evalexpression(arg0 : i32, arg1 :
                        i32, arg2 : i32,) -> i32 { unsafe { $($path_to_types)*::
                        _export_evalexpression_cabi::<$ty > (arg0, arg1, arg2) } } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_hyperlight_wasm_examples_calculator_calculate_cabi;
            }
        }
    }
}
#[rustfmt::skip]
mod _rt {
    #![allow(dead_code, clippy::all)]
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
}
/// Generates `#[unsafe(no_mangle)]` functions to export the specified type as
/// the root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_calculator_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::hyperlight_wasm_examples::calculator::calculate::__export_hyperlight_wasm_examples_calculator_calculate_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::hyperlight_wasm_examples::calculator::calculate);
    };
}
#[doc(inline)]
pub(crate) use __export_calculator_impl as export;
#[cfg(target_arch = "wasm32")]
#[unsafe(
    link_section = "component-type:wit-bindgen:0.41.0:hyperlight-wasm-examples:calculator:calculator:encoded world"
)]
#[doc(hidden)]
#[allow(clippy::octal_escapes)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 524] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x8b\x03\x01A\x02\x01\
A\x08\x01B\x02\x01@\x02\x01xy\x01yy\0y\x04\0\x03add\x01\0\x03\0'hyperlight-wasm-\
examples:calculator/add\x05\0\x01B\x02\x01@\x02\x01xy\x01yy\0y\x04\0\x08subtract\
\x01\0\x03\0,hyperlight-wasm-examples:calculator/subtract\x05\x01\x01B\x02\x01@\x02\
\x01xy\x01yy\0y\x04\0\x08multiply\x01\0\x03\0,hyperlight-wasm-examples:calculato\
r/multiply\x05\x02\x01B\x04\x01m\x03\x03add\x08subtract\x08multiply\x04\0\x02op\x03\
\0\0\x01@\x03\x02op\x01\x01xy\x01yy\0y\x04\0\x0eevalexpression\x01\x02\x04\0-hyp\
erlight-wasm-examples:calculator/calculate\x05\x03\x04\0.hyperlight-wasm-example\
s:calculator/calculator\x04\0\x0b\x10\x01\0\x0acalculator\x03\0\0\0G\x09producer\
s\x01\x0cprocessed-by\x02\x0dwit-component\x070.227.1\x10wit-bindgen-rust\x060.4\
1.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
