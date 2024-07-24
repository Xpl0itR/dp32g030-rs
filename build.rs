// Copyright © 2024 Xpl0itR
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate form;
extern crate svd2rust;

use std::{env, fs, path::Path};
use form::util::create_directory_structure;
use svd2rust::config::{IdentFormat, IdentFormatsTheme, SourceType, Target};

const SVD_PATH: &'static str = "src/DP32G030.svd";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let svd = fs::read_to_string(SVD_PATH).unwrap();

    let mut config = svd2rust::Config::default();
    config.target                = Target::CortexM;
    config.source_type           = SourceType::Xml;
    config.skip_crate_attributes = true;
    config.strict                = true;
    config.max_cluster_size      = true;
    config.impl_debug            = true;
    config.impl_debug_feature    = Some("defmt".into());
    config.ident_formats_theme   = Some(IdentFormatsTheme::Legacy);
    config.ident_formats.extend([
        ("register_spec".into(),   IdentFormat::default().constant_case().suffix("rs")),
        ("enum_name".into(),       IdentFormat::default().constant_case()),
        ("enum_read_name".into(),  IdentFormat::default().constant_case()),
        ("enum_write_name".into(), IdentFormat::default().constant_case()),
        ("enum_value".into(),      IdentFormat::default().pascal_case()),
    ]);

    let gen = svd2rust::generate(&svd, &config).unwrap();
    create_directory_structure(&out_dir, &gen.lib_rs, true).unwrap();

    if env::var_os("CARGO_FEATURE_RT").is_some() {
        if let Some(device) = gen.device_specific {
            fs::write(
                Path::new(&out_dir).join("device.x"),
                device.device_x).unwrap();
            println!("cargo::rustc-link-search={}", out_dir);
        }
    }

    println!("cargo::rerun-if-changed={}", SVD_PATH);
    println!("cargo::rerun-if-changed=build.rs");
}