/* This is the context used to generate index.html */

/* Standard Library */
use std::fs;
use std::path::Path;
use std::error::Error;

/* Third Party Libraries */
use serde::Serialize;
use rusqlite::Connection;
use user_error::UserError;

/* Crate Modules */
use crate::db;
use crate::web::templates;
use crate::post::EchoPost;
use crate::config::EchoConfig;

/* Constants */
const INDEX_LOCATION: &'static str = "dist/index.html";

#[derive(Serialize)]
pub struct EchoHomeContext<'a> {
    pub config: &'a EchoConfig,

    pub posts: Vec<Vec<EchoPost>>,
    pub num_posts: isize,
}

impl<'a> EchoHomeContext<'a> {
    pub fn new(config: &'a EchoConfig, connection: &Connection) -> Result<Self, UserError> {
        let posts = db::get_latest(&connection, 10)?;
        let num_posts = db::num_posts(&connection)?;

        Ok(EchoHomeContext {
            config,
            posts,
            num_posts,
        })
    }

    pub fn compile(&self) -> Result<String, UserError> {
        templates::compile_index(self)
    }

    pub fn write(&self, path: &AsRef<Path>) -> Result<(), UserError> {
        let template = self.compile()?;
        fs::write(path, &template).map_err(|e| {
            let s = path.as_ref().to_str().unwrap_or("<unknown>");
            let reason = format!("Could not write to: {}", s);
            UserError::hardcoded("Failed to generate the homepage",
                                 &[&reason, &e.description()], &[])
        })
    }
}

