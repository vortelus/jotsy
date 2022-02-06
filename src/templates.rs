/*
 * Copyright (c) 2022, Sayan Nandan <nandansayan@outlook.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/

use crate::handlers::app::Note;
use crate::util;
use askama::Template;
use axum::{body, http::StatusCode, response::Response};

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginPage {
    login_failed: bool,
}

impl LoginPage {
    pub fn new(login_failed: bool) -> String {
        Self { login_failed }.render().unwrap()
    }
}

#[derive(Template)]
#[template(path = "redirect.html")]
pub struct NoticePage {
    message: String,
    redirect: bool,
}

impl NoticePage {
    pub fn new(message: impl ToString, redirect: bool) -> String {
        NoticePage {
            message: message.to_string(),
            redirect,
        }
        .render()
        .unwrap()
    }
    pub fn new_redirect(message: impl ToString) -> String {
        Self::new(message, true)
    }
    pub fn e500() -> String {
        Self::new("An internal server error occurred", false)
    }
    pub fn e500_resp() -> Response {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(body::boxed(body::Full::from(NoticePage::e500())))
            .unwrap()
    }
    pub fn re500() -> crate::JotsyResponse {
        util::resp(StatusCode::INTERNAL_SERVER_ERROR, Self::e500())
    }
    pub fn empty() -> String {
        Self::new("", true)
    }
}

#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignupPage {
    error: Option<&'static str>,
}

impl SignupPage {
    pub fn new(message: &'static str) -> String {
        Self {
            error: Some(message),
        }
        .render()
        .unwrap()
    }
    pub fn empty() -> String {
        Self { error: None }.render().unwrap()
    }
}

#[derive(Template)]
#[template(path = "note.html", escape = "none")]
pub struct SingleNote {
    note: Note,
}

impl SingleNote {
    pub fn new(mut note: Note) -> String {
        // update markdown
        note.body = markdown::to_html(&note.body);
        Self { note }.render().unwrap()
    }
}

#[derive(Template)]
#[template(path = "app.html", escape = "none")]
pub struct App {
    username: String,
    count: usize,
    notes: Vec<Note>,
}

impl App {
    pub fn new(username: String, notes: Vec<Note>) -> String {
        Self {
            username,
            count: notes.len(),
            notes,
        }
        .render()
        .unwrap()
    }
}

#[derive(Template)]
#[template(path = "account.html")]
pub struct Account {
    count: u64,
    username: String,
}

impl Account {
    pub fn new(count: u64, username: String) -> String {
        Self { count, username }.render().unwrap()
    }
}
