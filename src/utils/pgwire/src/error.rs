// Copyright 2023 RisingWave Labs
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

use std::io::Error as IoError;

use thiserror::Error;

use crate::pg_server::BoxedError;
pub type PsqlResult<T> = std::result::Result<T, PsqlError>;

/// Error type used in pgwire crates.
#[derive(Error, Debug)]
pub enum PsqlError {
    #[error("Startup Error when connect to session: {0}")]
    StartupError(BoxedError),

    #[error("PasswordError: {0}")]
    PasswordError(IoError),

    #[error("QueryError: {0}")]
    QueryError(BoxedError),

    #[error("ParseError: {0}")]
    ParseError(BoxedError),

    #[error("ExecuteError: {0}")]
    ExecuteError(BoxedError),

    #[error("{0}")]
    IoError(#[from] IoError),

    #[error("{0}")]
    /// Include error for describe, bind.
    Internal(BoxedError),

    #[error("Panicked when processing: {0}.
This is a bug. We would appreciate a bug report at https://github.com/risingwavelabs/risingwave/issues/new?labels=type%2Fbug&template=bug_report.yml")]
    Panic(String),

    #[error("{0}")]
    SslError(String),
}

impl PsqlError {
    pub fn no_statement() -> Self {
        PsqlError::Internal("No statement found".into())
    }

    pub fn no_portal() -> Self {
        PsqlError::Internal("No portal found".into())
    }
}
