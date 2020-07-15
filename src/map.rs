use super::matchers;
use super::Matcher;

#[derive(Debug, Eq, PartialEq)]
pub enum MatcherType {
    APP,
    ARCHIVE,
    AUDIO,
    DOC,
    FONT,
    IMAGE,
    VIDEO,
    CUSTOM,
}

macro_rules! matcher_map {
    ($(($mtype:expr, $mime:literal, $ext:literal, $matcher:expr)),*) => {
        pub const MATCHER_MAP: &[(MatcherType, &'static str, &'static str, Matcher)] = &[
            $(($mtype, $mime, $ext, $matcher as Matcher),)*
        ];
    };
}

matcher_map!(
    (
        MatcherType::APP,
        "application/wasm",
        "wasm",
        matchers::app::is_wasm
    ),
    (
        MatcherType::APP,
        "application/x-executable",
        "elf",
        matchers::app::is_elf
    ),
    (
        MatcherType::APP,
        "application/vnd.microsoft.portable-executable",
        "exe",
        matchers::app::is_exe
    ),
    (
        MatcherType::APP,
        "application/java",
        "class",
        matchers::app::is_java
    ),
    (
        MatcherType::APP,
        "application/x-llvm",
        "bc",
        matchers::app::is_llvm
    ),
    // Image
    (
        MatcherType::IMAGE,
        "image/jpeg",
        "jpg",
        matchers::image::is_jpeg
    ),
    (
        MatcherType::IMAGE,
        "image/jp2",
        "jp2",
        matchers::image::is_jpeg2000
    ),
    (
        MatcherType::IMAGE,
        "image/png",
        "png",
        matchers::image::is_png
    ),
    (
        MatcherType::IMAGE,
        "image/gif",
        "gif",
        matchers::image::is_gif
    ),
    (
        MatcherType::IMAGE,
        "image/webp",
        "webp",
        matchers::image::is_webp
    ),
    (
        MatcherType::IMAGE,
        "image/x-canon-cr2",
        "cr2",
        matchers::image::is_cr2
    ),
    (
        MatcherType::IMAGE,
        "image/tiff",
        "tif",
        matchers::image::is_tiff
    ),
    (
        MatcherType::IMAGE,
        "image/bmp",
        "bmp",
        matchers::image::is_bmp
    ),
    (
        MatcherType::IMAGE,
        "image/vnd.ms-photo",
        "jxr",
        matchers::image::is_jxr
    ),
    (
        MatcherType::IMAGE,
        "image/vnd.adobe.photoshop",
        "psd",
        matchers::image::is_psd
    ),
    (
        MatcherType::IMAGE,
        "image/vnd.microsoft.icon",
        "ico",
        matchers::image::is_ico
    ),
    (
        MatcherType::IMAGE,
        "image/heif",
        "heif",
        matchers::image::is_heif
    ),
    (
        MatcherType::IMAGE,
        "image/avif",
        "avif",
        matchers::image::is_avif
    ),
    // Video
    (
        MatcherType::VIDEO,
        "video/mp4",
        "mp4",
        matchers::video::is_mp4
    ),
    (
        MatcherType::VIDEO,
        "video/x-m4v",
        "m4v",
        matchers::video::is_m4v
    ),
    (
        MatcherType::VIDEO,
        "video/x-matroska",
        "mkv",
        matchers::video::is_mkv
    ),
    (
        MatcherType::VIDEO,
        "video/webm",
        "webm",
        matchers::video::is_webm
    ),
    (
        MatcherType::VIDEO,
        "video/quicktime",
        "mov",
        matchers::video::is_mov
    ),
    (
        MatcherType::VIDEO,
        "video/x-msvideo",
        "avi",
        matchers::video::is_avi
    ),
    (
        MatcherType::VIDEO,
        "video/x-ms-wmv",
        "wmv",
        matchers::video::is_wmv
    ),
    (
        MatcherType::VIDEO,
        "video/mpeg",
        "mpg",
        matchers::video::is_mpeg
    ),
    (
        MatcherType::VIDEO,
        "video/x-flv",
        "flv",
        matchers::video::is_flv
    ),
    // Audio
    (
        MatcherType::AUDIO,
        "audio/midi",
        "midi",
        matchers::audio::is_midi
    ),
    (
        MatcherType::AUDIO,
        "audio/mpeg",
        "mp3",
        matchers::audio::is_mp3
    ),
    (
        MatcherType::AUDIO,
        "audio/m4a",
        "m4a",
        matchers::audio::is_m4a
    ),
    (
        MatcherType::AUDIO,
        "audio/ogg",
        "ogg",
        matchers::audio::is_ogg
    ),
    (
        MatcherType::AUDIO,
        "audio/x-flac",
        "flac",
        matchers::audio::is_flac
    ),
    (
        MatcherType::AUDIO,
        "audio/x-wav",
        "wav",
        matchers::audio::is_wav
    ),
    (
        MatcherType::AUDIO,
        "audio/amr",
        "amr",
        matchers::audio::is_amr
    ),
    (
        MatcherType::AUDIO,
        "audio/aac",
        "aac",
        matchers::audio::is_aac
    ),
    // Font
    (
        MatcherType::FONT,
        "application/font-woff",
        "woff",
        matchers::font::is_woff
    ),
    (
        MatcherType::FONT,
        "application/font-woff",
        "woff2",
        matchers::font::is_woff2
    ),
    (
        MatcherType::FONT,
        "application/font-sfnt",
        "ttf",
        matchers::font::is_ttf
    ),
    (
        MatcherType::FONT,
        "application/font-sfnt",
        "otf",
        matchers::font::is_otf
    ),
    // Document
    (
        MatcherType::DOC,
        "application/msword",
        "doc",
        matchers::doc::is_doc
    ),
    (
        MatcherType::DOC,
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "docx",
        matchers::doc::is_docx
    ),
    (
        MatcherType::DOC,
        "application/vnd.ms-excel",
        "xls",
        matchers::doc::is_xls
    ),
    (
        MatcherType::DOC,
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "xlsx",
        matchers::doc::is_xlsx
    ),
    (
        MatcherType::DOC,
        "application/vnd.ms-powerpoint",
        "ppt",
        matchers::doc::is_ppt
    ),
    (
        MatcherType::DOC,
        "application/application/vnd.openxmlformats-officedocument.presentationml.presentation",
        "pptx",
        matchers::doc::is_pptx
    ),
    // Archive
    (
        MatcherType::ARCHIVE,
        "application/epub+zip",
        "epub",
        matchers::archive::is_epub
    ),
    (
        MatcherType::ARCHIVE,
        "application/zip",
        "zip",
        matchers::archive::is_zip
    ),
    (
        MatcherType::ARCHIVE,
        "application/x-tar",
        "tar",
        matchers::archive::is_tar
    ),
    (
        MatcherType::ARCHIVE,
        "application/vnd.rar",
        "rar",
        matchers::archive::is_rar
    ),
    (
        MatcherType::ARCHIVE,
        "application/gzip",
        "gz",
        matchers::archive::is_gz
    ),
    (
        MatcherType::ARCHIVE,
        "application/x-bzip2",
        "bz2",
        matchers::archive::is_bz2
    ),
    (
        MatcherType::ARCHIVE,
        "application/x-7z-compressed",
        "7z",
        matchers::archive::is_7z
    ),
    (
        MatcherType::ARCHIVE,
        "application/x-xz",
        "xz",
        matchers::archive::is_xz
    ),
    (
        MatcherType::ARCHIVE,
        "application/pdf",
        "pdf",
        matchers::archive::is_pdf
    ),
    (
        MatcherType::ARCHIVE,
        "application/x-shockwave-flash",
        "swf",
        matchers::archive::is_swf
    ),
    (
        MatcherType::ARCHIVE,
        "application/rtf",
        "rtf",
        matchers::archive::is_rtf
    ),
    (
        MatcherType::ARCHIVE,
        "application/octet-stream",
        "eot",
        matchers::archive::is_eot
    ),
    (
        MatcherType::ARCHIVE,
        "application/postscript",
        "ps",
        matchers::archive::is_ps
    ),
    (
        MatcherType::ARCHIVE,
        "application/vnd.sqlite3",
        "sqlite",
        matchers::archive::is_sqlite
    ),
    (
        MatcherType::ARCHIVE,
        "application/x-nintendo-nes-rom",
        "nes",
        matchers::archive::is_nes
    ),
    (
        MatcherType::ARCHIVE,
        "application/x-google-chrome-extension",
        "crx",
        matchers::archive::is_crx
    ),
    (
        MatcherType::ARCHIVE,
        "application/vnd.ms-cab-compressed",
        "cab",
        matchers::archive::is_cab
    ),
    (
        MatcherType::ARCHIVE,
        "application/vnd.debian.binary-package",
        "deb",
        matchers::archive::is_deb
    ),
    (
        MatcherType::ARCHIVE,
        "application/x-unix-archive",
        "ar",
        matchers::archive::is_ar
    ),
    (
        MatcherType::ARCHIVE,
        "application/x-compress",
        "Z",
        matchers::archive::is_z
    ),
    (
        MatcherType::ARCHIVE,
        "application/x-lzip",
        "lz",
        matchers::archive::is_lz
    ),
    (
        MatcherType::ARCHIVE,
        "application/x-rpm",
        "rpm",
        matchers::archive::is_rpm
    ),
    (
        MatcherType::ARCHIVE,
        "application/dicom",
        "dcm",
        matchers::archive::is_dcm
    ),
    (
        MatcherType::ARCHIVE,
        "application/zstd",
        "zst",
        matchers::archive::is_zst
    )
);
