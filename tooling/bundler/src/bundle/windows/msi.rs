// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

mod wix;

use crate::Settings;

use std::{self, path::PathBuf};

/// Runs all of the commands to build the MSI installer.
/// Returns a vector of PathBuf that shows where the MSI was created.
pub fn bundle_project(settings: &Settings) -> crate::Result<Vec<PathBuf>> {
  let mut wix_path = dirs_next::cache_dir().unwrap();
  wix_path.push("tauri/WixTools");

  if !wix_path.exists() {
    wix::get_and_extract_wix(&wix_path)?;
  }

  wix::build_wix_app_installer(settings, &wix_path)
}
