use once_cell::sync::Lazy as LazyLock;

use crate::util::{
    fxhash::FxHashMap,
    string::{LowercaseString, SmallString},
};

pub static EMACS_CHARSET_MAP: LazyLock<FxHashMap<SmallString, (Vec<u32>, Option<SmallString>)>> =
    LazyLock::new(|| {
        log::trace!("EMACS_CHARSET_MAP is being created...");
        let mut charset_map = FxHashMap::default();
        charset_map.insert(
            SmallString::new("iso8859-1"),
            (vec![0x00A0, 0x00A1, 0x00B4, 0x00BC, 0x00D0], None),
        );
        charset_map.insert(SmallString::new("iso8859-2"), (vec![0x00A0, 0x010E], None));
        charset_map.insert(SmallString::new("iso8859-3"), (vec![0x00A0, 0x0108], None));
        charset_map.insert(
            SmallString::new("iso8859-4"),
            (vec![0x00A0, 0x00AF, 0x0128, 0x0156, 0x02C7], None),
        );
        charset_map.insert(SmallString::new("iso8859-5"), (vec![0x00A0, 0x0401], None));
        charset_map.insert(SmallString::new("iso8859-6"), (vec![0x00A0, 0x060C], None));
        charset_map.insert(SmallString::new("iso8859-7"), (vec![0x00A0, 0x0384], None));
        charset_map.insert(SmallString::new("iso8859-8"), (vec![0x00A0, 0x05D0], None));
        charset_map.insert(
            SmallString::new("iso8859-9"),
            (vec![0x00A0, 0x00A1, 0x00BC, 0x011E], None),
        );
        charset_map.insert(
            SmallString::new("iso8859-10"),
            (vec![0x00A0, 0x00D0, 0x0128, 0x2015], None),
        );
        charset_map.insert(SmallString::new("iso8859-11"), (vec![0x00A0, 0x0E01], None));
        charset_map.insert(SmallString::new("iso8859-13"), (vec![0x00A0, 0x201C], None));
        charset_map.insert(SmallString::new("iso8859-14"), (vec![0x00A0, 0x0174], None));
        charset_map.insert(
            SmallString::new("iso8859-15"),
            (vec![0x00A0, 0x00A1, 0x00D0, 0x0152], None),
        );
        charset_map.insert(SmallString::new("iso8859-16"), (vec![0x00A0, 0x0218], None));
        charset_map.insert(
            SmallString::new("gb2312.1980-0"),
            (vec![0x4E13], Some(SmallString::new("zh-Hans"))),
        );

        charset_map.insert(
            SmallString::new("big5-0"),
            (
                vec![/* 0x9C21 in ftfont.c */ 0x4EDC],
                Some(SmallString::new("zh-Hant")),
            ),
        );
        charset_map.insert(
            SmallString::new("jisx0208.1983-0"),
            (vec![0x4E55], Some(SmallString::new("ja"))),
        );
        charset_map.insert(
            SmallString::new("ksc5601.1987-0"),
            (vec![0xAC00], Some(SmallString::new("ko"))),
        );
        charset_map.insert(
            SmallString::new("cns11643.1992-1"),
            (vec![0xFE32], Some(SmallString::new("zh-Hant"))),
        );
        charset_map.insert(
            SmallString::new("cns11643.1992-2"),
            (vec![0x4E33, 0x7934], None),
        );
        charset_map.insert(SmallString::new("cns11643.1992-3"), (vec![0x201A9], None));
        charset_map.insert(SmallString::new("cns11643.1992-4"), (vec![0x20057], None));
        charset_map.insert(SmallString::new("cns11643.1992-5"), (vec![0x20000], None));
        charset_map.insert(SmallString::new("cns11643.1992-6"), (vec![0x20003], None));
        charset_map.insert(SmallString::new("cns11643.1992-7"), (vec![0x20055], None));
        charset_map.insert(
            SmallString::new("gbk-0"),
            (vec![0x4E06], Some(SmallString::new("zh-Hans"))),
        );
        charset_map.insert(SmallString::new("jisx0212.1990-0"), (vec![0x4E44], None));
        charset_map.insert(
            SmallString::new("jisx0213.2000-1"),
            (vec![0xFA10], Some(SmallString::new("ja"))),
        );
        charset_map.insert(SmallString::new("jisx0213.2000-2"), (vec![0xFA49], None));
        charset_map.insert(SmallString::new("jisx0213.2004-1"), (vec![0x20B9F], None));
        charset_map.insert(
            SmallString::new("viscii1.1-1"),
            (vec![0x1EA0, 0x1EAE, 0x1ED2], Some(SmallString::new("vi"))),
        );
        charset_map.insert(
            SmallString::new("tis620.2529-1"),
            (vec![0x0E01], Some(SmallString::new("th"))),
        );
        charset_map.insert(
            SmallString::new("windows-1251"),
            (vec![0x0401, 0x0490], Some(SmallString::new("ru"))),
        );
        charset_map.insert(
            SmallString::new("koi8-r"),
            (vec![0x0401, 0x2219], Some(SmallString::new("ru"))),
        );
        charset_map.insert(
            SmallString::new("mulelao-1"),
            (vec![0x0E81], Some(SmallString::new("lo"))),
        );
        charset_map.insert(SmallString::new("unicode-sip"), (vec![0x20000], None));

        charset_map
    });
