// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate base64;
extern crate clap;
extern crate env_logger;
extern crate habitat_core as hcore;
extern crate habitat_common as common;
extern crate habitat_pkg_export_docker as export_docker;
extern crate chrono;
#[macro_use]
extern crate log;

use clap::App;
use common::ui::UI;
use hcore::PROGRAM_NAME;

use export_docker::{Cli, PkgIdentArgOptions, Result};

fn main() {
    env_logger::init().unwrap();
    let mut ui = UI::default_with_env();
    if let Err(e) = start(&mut ui) {
        ui.fatal(e).unwrap();
        std::process::exit(1)
    }
}

fn start(ui: &mut UI) -> Result<()> {
    let cli = cli();
    let m = cli.get_matches();
    debug!("clap cli args: {:?}", m);

    export_docker::export_for_cli_matches(ui, &m)
}

fn cli<'a, 'b>() -> App<'a, 'b> {
    let name: &str = &*PROGRAM_NAME;
    let about = "Creates (an optionally pushes) a Docker image from a set of Habitat packages";

    Cli::new(name, about)
        .add_base_packages_args()
        .add_builder_args()
        .add_image_customization_args()
        .add_tagging_args()
        .add_publishing_args()
        .add_pkg_ident_arg(PkgIdentArgOptions { multiple: true })
        .app
}
