/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod JS {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub use root::_bindgen_ty_1 as JSWhyMagic;
        #[repr(C)]
        #[derive(Debug, Copy)]
        pub struct Value {
            pub _address: u8,
        }
        #[test]
        fn bindgen_test_layout_Value() {
            assert_eq!(::std::mem::size_of::<Value>() , 1usize);
            assert_eq!(::std::mem::align_of::<Value>() , 1usize);
        }
        extern "C" {
            #[link_name = "_ZN2JS5Value1aE10JSWhyMagic"]
            pub fn Value_a(this: *mut root::JS::Value,
                           arg1: root::JS::JSWhyMagic);
        }
        impl Clone for Value {
            fn clone(&self) -> Self { *self }
        }
        impl Value {
            #[inline]
            pub unsafe fn a(&mut self, arg1: root::JS::JSWhyMagic) {
                Value_a(&mut *self, arg1)
            }
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum _bindgen_ty_1 { }
}