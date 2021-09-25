use super::json_package_configuration::JsonPackageConfiguration;
use crate::{package_script_finder, FilePathConverter};
use std::{collections::HashMap, error::Error, sync::Arc};

pub struct JsonPackageConfigurationReader {
    file_system: Arc<dyn app::infra::FileSystem>,
    file_path_converter: Arc<FilePathConverter>,
    build_configuration_filename: &'static str,
    ffi_build_script_basename: &'static str,
}

impl JsonPackageConfigurationReader {
    pub fn new(
        file_system: Arc<dyn app::infra::FileSystem>,
        file_path_converter: Arc<FilePathConverter>,
        build_configuration_filename: &'static str,
        ffi_build_script_basename: &'static str,
    ) -> Self {
        Self {
            file_system,
            file_path_converter,
            build_configuration_filename,
            ffi_build_script_basename,
        }
    }
}

impl app::infra::PackageConfigurationReader for JsonPackageConfigurationReader {
    fn get_dependencies(
        &self,
        package_directory: &app::infra::FilePath,
    ) -> Result<HashMap<String, url::Url>, Box<dyn Error>> {
        Ok(
            serde_json::from_str::<JsonPackageConfiguration>(&self.file_system.read_to_string(
                &package_directory.join(&app::infra::FilePath::new(vec![
                    self.build_configuration_filename,
                ])),
            )?)?
            .get_dependencies()?,
        )
    }

    fn is_ffi_enabled(
        &self,
        package_directory: &app::infra::FilePath,
    ) -> Result<bool, Box<dyn Error>> {
        Ok(package_script_finder::find(
            &self
                .file_path_converter
                .convert_to_os_path(package_directory),
            self.ffi_build_script_basename,
        )?
        .is_some())
    }
}