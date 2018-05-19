// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The Rust Language Server.
//!
//! The RLS provides a server that runs in the background, providing IDEs,
//! editors, and other tools with information about Rust programs. It supports
//! functionality such as 'goto definition', symbol search, reformatting, and
//! code completion, and enables renaming and refactorings.

#![feature(rustc_private)]
#![feature(integer_atomics)]
#![feature(drain_filter)]

#![allow(unknown_lints)]
#![warn(clippy)]
#![allow(cyclomatic_complexity, needless_pass_by_value, too_many_arguments)]

extern crate cargo;
extern crate cargo_metadata;
extern crate env_logger;
#[macro_use]
extern crate failure;
extern crate jsonrpc_core;
extern crate languageserver_types as ls_types;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate racer;
extern crate rayon;
extern crate rls_analysis as analysis;
extern crate rls_blacklist as blacklist;
extern crate rls_data as data;
extern crate rls_rustc as rustc_shim;
extern crate rls_span as span;
extern crate rls_vfs as vfs;
extern crate rustfmt_nightly as rustfmt;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

extern crate url;
extern crate walkdir;

use std::env;
use std::sync::Arc;

pub mod actions;
pub mod build;
pub mod config;
pub mod lsp_data;
pub mod server;

const RUSTC_SHIM_ENV_VAR_NAME: &str = "RLS_RUSTC_SHIM";

type Span = span::Span<span::ZeroIndexed>;

pub fn main() {
    use server::io::StdioOutput;
    let output = StdioOutput::new();

    let a = Arc::new(analysis::AnalysisHost::new(analysis::Target::Debug));
    let v = Arc::new(vfs::Vfs::new());
    
    use std::sync::Mutex;
    use config::Config;
    let config = Arc::new(Mutex::new(Config::default()));
    
    use actions::ActionContext;
    //let mut ctx = ActionContext::Uninit(actions::UninitActionContext {
    //    analysis: a, 
    //    vfs: v, 
    //    config: config });
    use std;
    let workspace_root_path = std::path::Path::new("/Users/Eonil/Workshop/Playground/rust-query-analysis/example1")
        .to_path_buf();
    //ctx.init(workspace_root_path, &output);
    let mut ctx = actions::InitActionContext::new(a, v, config, workspace_root_path);
    ctx.init(&output);
    println!("done!");
}

