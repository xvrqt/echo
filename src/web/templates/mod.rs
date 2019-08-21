/* Third Party Libraries */
use tera::{GlobalFn, Tera, Value};
use user_error::UserError;

/* Internal Modules */
use crate::context::EchoContext;

/* Returns a Value::String with the suffix for a date ending */
fn date_suffix(n: i64) -> Result<Value, tera::Error> {
    if 11 <= n && n <= 13 {
        Ok(Value::String(String::from("th")))
    } else {
        let n = n % 10;
        match n {
            1 => Ok(Value::String(String::from("st"))),
            2 => Ok(Value::String(String::from("nd"))),
            3 => Ok(Value::String(String::from("rd"))),
            _ => Ok(Value::String(String::from("th"))),
        }
    }
}

/* Registered with Tera to provide in template date suffixing */
fn date_suffix_wrapper() -> GlobalFn {
    Box::new(move |args| -> Result<Value, tera::Error> {
        const ERROR_SUMMARY: &str = "Failed to generate date suffix";
        /* Make sure we have the correct key */
        match args.get("timestamp") {
            Some(ts) => match ts {
                Value::Number(n) if n.is_i64() => date_suffix(n.as_i64().unwrap()),
                _ => Err(ERROR_SUMMARY.into()),
            },
            None => Err(ERROR_SUMMARY.into()),
        }
    })
}

/* Compiles the index.html and returns it as a String */
pub fn compile_index(context: &EchoContext) -> Result<String, UserError> {
    let template_glob = concat!(env!("CARGO_MANIFEST_DIR"), "/src/web/templates/**/*");
    match Tera::new(template_glob).and_then(|mut t| {
        t.register_function("date_suffix", date_suffix_wrapper());
        t.render("index.html", &context)
    }) {
        Err(e) => Err(UserError::hardcoded(
            "Failed to compile templates",
            &[&e.to_string()],
            &[],
        )),
        Ok(s) => Ok(s),
    }
}
