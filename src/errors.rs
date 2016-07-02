// Copyright 2014 Google Inc. All rights reserved.
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

use std::{error, fmt};

pub use self::hat_error::HatError;
pub use self::diesel_error::DieselError;

#[derive(Clone, Copy, Debug)]
pub enum LockError {
    Poisoned,
    RequestLimitReached,
}

impl fmt::Display for LockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        (self as &fmt::Debug).fmt(f)
    }
}

impl error::Error for LockError {
    fn description(&self) -> &str {
        match *self {
            LockError::Poisoned => "Poisoned",
            LockError::RequestLimitReached => "Request limit reached",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RetryError {
    Poisoned,
    RequestLimitReached,
    Retry,
}

impl fmt::Display for RetryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        (self as &fmt::Debug).fmt(f)
    }
}

impl error::Error for RetryError {
    fn description(&self) -> &str {
        match *self {
            RetryError::Poisoned => "Poisoned",
            RetryError::RequestLimitReached => "Request limit reached",
            RetryError::Retry => "Retry request",
        }
    }
}

impl From<LockError> for RetryError {
    fn from(e: LockError) -> RetryError {
        match e {
            LockError::Poisoned => RetryError::Poisoned,
            LockError::RequestLimitReached => RetryError::RequestLimitReached,
        }
    }
}

mod hat_error {
    use std::{io, str};
    use std::borrow::Cow;
    use std::sync::mpsc;
    use capnp;

    use blob;
    use key;

    error_type! {
        #[derive(Debug)]
        pub enum HatError {
            Recv(mpsc::RecvError) {
                cause;
            },
            Keys(key::MsgError) {
                cause;
            },
            Blobs(blob::MsgError) {
                cause;
            },
            DataSerialization(capnp::Error) {
                cause;
            },
            IO(io::Error) {
                cause;
            },
            Message(Cow<'static, str>) {
                desc (e) &**e;
                from (s: &'static str) s.into();
                from (s: String) s.into();
            },
            LockError(super::LockError) {
                cause;
            },
            DieselError(super::DieselError) {
                cause;
            }
        }
    }
}

mod diesel_error {
    use diesel;

    error_type! {
        #[derive(Debug)]
        pub enum DieselError {
            SqlConnection(diesel::ConnectionError) {
                cause;
            },
            SqlMigration(diesel::migrations::MigrationError) {
                cause;
            },
            SqlRunMigration(diesel::migrations::RunMigrationsError) {
                cause;
            },
            SqlExecute(diesel::result::Error) {
                cause;
            },
        }
    }
}