#[macro_export]
macro_rules! resource_table {
    {$($(#[$attr:meta])* $v:vis static $i:ident : $t:ty = $e:expr;)*} => {
        #[doc(hidden)]
        mod _resource_table {
            use super::*;

            #[repr(C)]
            #[allow(nonstandard_style)]
            pub struct _ResourceTable {
                _resource_table_header: $crate::types::Header<{
                    [$(stringify!($i),)*].len()
                }>,
                $(pub $i: $crate::types::Resource<$t>,)*
            }

            #[unsafe(link_section = ".resource_table")]
            #[unsafe(export_name = "_RESOURCE_TABLE")]
            #[allow(dead_code)]
            pub static _RESOURCE_TABLE: _ResourceTable = _ResourceTable {
                _resource_table_header: $crate::types::Header::new([
                    $(core::mem::offset_of!(_ResourceTable, $i) as u32,)*
                ]),
                $($i: {
                    assert!(
                        core::mem::align_of::<$t>() <= 4,
                        concat!(
                            "resource ",
                            stringify!($t),
                            " has bad alignment",
                        ),
                    );
                    $crate::types::Resource::new($e)
                },)*
            };
        }

        $(
            #[allow(dead_code)]
            $(#[$attr])*
            $v static $i: &$t = &_resource_table::_RESOURCE_TABLE.$i.data;
        )*
    };
}
