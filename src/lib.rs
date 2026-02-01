#![no_std]

// useful references:
// https://github.com/torvalds/linux/blob/master/include/linux/remoteproc.h
// https://github.com/OpenAMP/open-amp/blob/main/lib/include/openamp/remoteproc.h

pub mod constants;
mod r#macro;
pub mod types;
mod util;

pub use types::{DevAddr, DevBuf};

mod carveout;
pub use carveout::Carveout;

#[cfg(test)]
mod test {
    use super::*;

    resource_table! {
        /// Documentation comment.
        pub static CARVEOUT: Carveout =
            Carveout::new_dynamic(0x8000, 0, "carveout");
        static CARVEOUT2: Carveout =
            Carveout::new_dynamic(0x4000, 1, "outcarve");
    }

    #[test]
    fn test_name() {
        assert!(CARVEOUT.name == util::str_to_array("carveout").unwrap());
    }
}
