// Copyright 2017 CoreOS, Inc.
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

#[macro_use]
extern crate hyper;
extern crate reqwest;

#[macro_use]
extern crate slog;
#[macro_use]
extern crate slog_scope;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

extern crate pnet;

extern crate openssl;

extern crate users;

#[macro_use]
mod macros;
mod providers;
mod metadata;
mod ssh;
mod network;
mod retry;

use providers::*;
use metadata::Metadata;

/// fetch_metadata is the generic, top-level function that is used by the main
/// function to fetch metadata. The configured provider is passed in and this
/// function dispatches the call to the correct provider-specific fetch function
pub fn fetch_metadata(provider: &str) -> Result<Metadata, String> {
    match provider {
        "azure" => azure::fetch_metadata(),
        "digitalocean" => digitalocean::fetch_metadata(),
        "ec2" => ec2::fetch_metadata(),
        "gce" => gce::fetch_metadata(),
        "openstack" => openstack::fetch_metadata(),
        "packet" => packet::fetch_metadata(),
        "vagrant_virtualbox" => vagrant_virtualbox::fetch_metadata(),
        _ => Err(format!("unknown provider '{}'", provider)),
    }
}
