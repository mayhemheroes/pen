pub const BUILD_CONFIGURATION_FILENAME: &str = "pen.json";
pub const BIT_CODE_FILE_EXTENSION: &str = "bc";

pub const LANGUAGE_ROOT_HOST_NAME: &str = "pen-root";
pub const LANGUAGE_ROOT_ENVIRONMENT_VARIABLE: &str = "PEN_ROOT";

pub const DEFAULT_SYSTEM_PACKAGE_URL: &str = "file://pen-root/lib/os";
pub const PRELUDE_PACKAGE_URL: &str = "file://pen-root/lib/prelude";
pub const FFI_BUILD_SCRIPT: &str = "pen-ffi";

pub const FILE_PATH_CONFIGURATION: app::infra::FilePathConfiguration =
    app::infra::FilePathConfiguration {
        source_file_extension: "pen",
        object_file_extension: "o",
        interface_file_extension: "i",
        archive_file_extension: "a",
        build_script_file_extension: "ninja",
    };