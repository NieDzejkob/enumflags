use enumflags2_derive::EnumFlags;

include!("../common.rs");

#[test]
fn module() {
    mod some_modules {
        #[derive(enumflags2_derive::EnumFlags, Copy, Clone, Debug)]
        #[repr(u8)]
        enum Test2 {
            A = 1 << 0,
            B = 1 << 1,
            C = 1 << 2,
            D = 1 << 3,
        }
    }
}
