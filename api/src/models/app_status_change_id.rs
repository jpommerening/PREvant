/*-
 * ========================LICENSE_START=================================
 * PREvant REST API
 * %%
 * Copyright (C) 2018 - 2019 aixigo AG
 * %%
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 * =========================LICENSE_END==================================
 */
use http_api_problem::{HttpApiProblem, StatusCode};
use rocket::http::RawStr;
use rocket::request::FromParam;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AppStatusChangeId(uuid::Uuid);

impl AppStatusChangeId {
    pub fn new() -> Self {
        AppStatusChangeId(uuid::Uuid::new_v4())
    }
}

impl Default for AppStatusChangeId {
    fn default() -> Self {
        AppStatusChangeId::new()
    }
}

impl std::fmt::Display for AppStatusChangeId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.to_hyphenated())
    }
}

impl std::str::FromStr for AppStatusChangeId {
    type Err = AppStatusChangeIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AppStatusChangeId(uuid::Uuid::from_str(s)?))
    }
}

impl<'r> FromParam<'r> for AppStatusChangeId {
    type Error = AppStatusChangeIdError;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        AppStatusChangeId::from_str(&param.url_decode()?)
    }
}

#[derive(Debug, Fail)]
pub enum AppStatusChangeIdError {
    #[fail(display = "Failed to parse UUID: {}", err)]
    UuidError { err: String },
    #[fail(display = "Invalid url encoded parameter: {}", err)]
    InvalidUrlDecodedParam { err: String },
}

impl From<std::str::Utf8Error> for AppStatusChangeIdError {
    fn from(err: std::str::Utf8Error) -> Self {
        AppStatusChangeIdError::InvalidUrlDecodedParam {
            err: format!("{}", err),
        }
    }
}

impl From<uuid::Error> for AppStatusChangeIdError {
    fn from(err: uuid::Error) -> Self {
        AppStatusChangeIdError::UuidError {
            err: format!("{}", err),
        }
    }
}

impl From<AppStatusChangeIdError> for HttpApiProblem {
    fn from(err: AppStatusChangeIdError) -> Self {
        HttpApiProblem::with_title_from_status(StatusCode::BAD_REQUEST)
            .set_detail(format!("{}", err))
    }
}
