// This code is taken from https://github.com/diggsey/bevy_clap, which is licensed under either of
//
// * Apache License, Version 2.0, (../LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
// * MIT license (../LICENSE-MIT or http://opensource.org/licenses/MIT)
//
// at your option.

// Copied and not used as a dependency because it uses bevy 0.6

#![allow(dead_code)]

use std::{
    any::type_name,
    ffi::OsString,
    fmt::{self, Debug, Formatter},
    marker::PhantomData,
};

use bevy::{ecs::system::Resource, prelude::Plugin};
use clap::Parser;

pub use clap;

/// A plugin which parses command line arguments into a struct, and inserts
/// the resulting struct as a resource.
pub struct ClapPlugin<T> {
    override_args: Option<Vec<OsString>>,
    phantom: PhantomData<fn() -> T>,
}

impl<T> Default for ClapPlugin<T> {
    fn default() -> Self {
        Self {
            override_args: None,
            phantom: PhantomData,
        }
    }
}

impl<T> Debug for ClapPlugin<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClapPlugin")
            .field("override_args", &self.override_args)
            .field("type_name", &type_name::<T>())
            .finish()
    }
}

impl<T> ClapPlugin<T> {
    /// Provide command-line arguments directly instead of using `std::env::args_os`.
    pub fn with_args<I: Into<OsString>>(args: impl IntoIterator<Item = I>) -> Self {
        Self {
            override_args: Some(args.into_iter().map(Into::into).collect()),
            phantom: PhantomData,
        }
    }
}

impl<T: Parser + Resource> Plugin for ClapPlugin<T> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(if let Some(args) = self.override_args.clone() {
            T::parse_from(args)
        } else {
            T::parse()
        });
    }
}
