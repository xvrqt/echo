/* Collection of useful database functions to keep things DRY */

/* Third Party Libraries */
use rusqlite::{Connection, ToSql, NO_PARAMS};
use user_error::UserError;

/* Crate Modules */
use crate::post::EchoPost;

/* Constants */
const DATABASE_FILE: &'static str = "echo.db";

/* Returns a Connection to the SQLite database.
   Assumes you're in the root directory of an Echo project.
*/
pub fn connect() -> Result<Connection, UserError> {
    let db_file_path = DATABASE_FILE;
    let c = Connection::open(db_file_path)?;
    Ok(c)
}

/* Creates a new post with the content provided */
pub fn new_post(c: Connection, text: &str) -> Result<(), UserError> {
    c.execute(
        "INSERT INTO posts (created, edited, text)
             VALUES (
                strftime('%s','now'),
                strftime('%s','now'),
                (?1)
            )",
        &[&text],
    )?;

    Ok(())
}
/* Returns and EchoPost of the given ID */
pub fn get_post(c: &Connection, id: isize) -> Result<EchoPost, UserError> {
    type MappedRows = Vec<Result<EchoPost, rusqlite::Error>>;

    let s = &format!(
        "SELECT * \
         FROM posts \
         ORDER BY created DESC \
         LIMIT 1 \
         OFFSET {}",
        id
    );
    let mut stmt = c.prepare(&s)?;

    let results = stmt.query_map(NO_PARAMS, |row| {
        Ok(EchoPost {
            id: row.get(0)?,
            created: row.get(1)?,
            edited: row.get(2)?,
            text: row.get(3)?,
        })
    })?;

    let (posts, errors): (MappedRows, MappedRows) = results.partition(|r| r.is_ok());

    /* Check for errors and concatenate into a single UserError */
    if !errors.is_empty() {
        let error = format!("Experienced an error in {} posts", errors.len());
        let mut ue = UserError::hardcoded("Failed to fetch latest posts", &[&error], &[]);
        for error in errors {
            let error = error.err().unwrap().to_string();
            ue.add_reason(&error);
        }
        return Err(ue);
    }

    /* Unwrap the posts */
    let posts: Vec<EchoPost> = posts.into_iter().map(|p| p.unwrap()).collect();
    if posts.len() != 1 {
        let summary = format!("Failed to find post #{}", id);
        return Err(UserError::simple(&summary));
    }

    Ok(posts[0].clone())
}

/* Updates and existing post */
pub fn delete_post(c: &Connection, id: isize) -> Result<(), UserError> {
    let query = "DELETE \
                 FROM posts \
                 ORDER BY created DESC \
                 LIMIT 1 OFFSET ?";
    let mut stmt = c.prepare(&query)?;
    stmt.execute(&[&id])?;

    Ok(())
}

/* Updates and existing post */
pub fn update_post(c: &Connection, post: EchoPost) -> Result<(), UserError> {
    let query = "UPDATE posts \
                 SET \
                 created = ?, \
                 edited  = ?, \
                 text    = ? \
                 WHERE \
                 id = ?";
    let mut stmt = c.prepare(&query)?;
    let values: [&ToSql; 4] = [&post.created, &post.edited, &post.text, &post.id];
    stmt.execute(&values)?;

    Ok(())
}

/* Returns the latest 'num' posts */
pub fn get_latest(c: &Connection, num: usize) -> Result<Vec<Vec<EchoPost>>, UserError> {
    type MappedRows = Vec<Result<EchoPost, rusqlite::Error>>;

    let s = format!("SELECT * FROM posts ORDER BY created DESC LIMIT {}", num);
    let mut stmt = c.prepare(&s)?;
    let results = stmt.query_map(NO_PARAMS, |row| {
        Ok(EchoPost {
            id: row.get(0)?,
            created: row.get(1)?,
            edited: row.get(2)?,
            text: row.get(3)?,
        })
    })?;

    let (posts, errors): (MappedRows, MappedRows) = results.partition(|r| r.is_ok());

    /* Check for errors and concatenate into a single UserError */
    if !errors.is_empty() {
        let error = format!("Experienced an error in {} posts", errors.len());
        let mut ue = UserError::hardcoded("Failed to fetch latest posts", &[&error], &[]);
        for error in errors {
            let error = error.err().unwrap().to_string();
            ue.add_reason(&error);
        }
        return Err(ue);
    }

    /* Unwrap the posts, organize into days */
    let posts: Vec<EchoPost> = posts.into_iter().map(|p| p.unwrap()).collect();
    let mut vov_posts: Vec<Vec<EchoPost>> = Vec::with_capacity(posts.len());
    let mut current_day = 0;
    for post in posts {
        let day_created = post.created / (24 * 60 * 60);
        if day_created != current_day {
            vov_posts.push(Vec::new());
            current_day = day_created;
        }
        if let Some(v) = vov_posts.last_mut() {
            v.push(post);
        }
    }

    Ok(vov_posts)
}


/* Returns all the posts */
pub fn get_all(c: &Connection) -> Result<Vec<Vec<EchoPost>>, UserError> {
    type MappedRows = Vec<Result<EchoPost, rusqlite::Error>>;

    let mut stmt = c.prepare("SELECT * FROM posts ORDER BY created DESC")?;
    let results = stmt.query_map(NO_PARAMS, |row| {
        Ok(EchoPost {
            id: row.get(0)?,
            created: row.get(1)?,
            edited: row.get(2)?,
            text: row.get(3)?,
        })
    })?;

    let (posts, errors): (MappedRows, MappedRows) = results.partition(|r| r.is_ok());

    /* Check for errors and concatenate into a single UserError */
    if !errors.is_empty() {
        let error = format!("Experienced an error in {} posts", errors.len());
        let mut ue = UserError::hardcoded("Failed to fetch latest posts", &[&error], &[]);
        for error in errors {
            let error = error.err().unwrap().to_string();
            ue.add_reason(&error);
        }
        return Err(ue);
    }

    /* Unwrap the posts */
    let posts: Vec<EchoPost> = posts.into_iter().map(|p| p.unwrap()).collect();
    let mut vov_posts: Vec<Vec<EchoPost>> = Vec::with_capacity(posts.len());
    let mut current_day = 0;
    for post in posts {
        let day_created = post.created / (24 * 60 * 60);
        if day_created != current_day {
            vov_posts.push(Vec::new());
            current_day = day_created;
        }
        if let Some(v) = vov_posts.last_mut() {
            v.push(post);
        }
    }

    Ok(vov_posts)
}

/* Returns the number of posts */
pub fn num_posts(c: &Connection) -> Result<isize, UserError> {
    let mut stmt = c.prepare("SELECT COUNT(*) FROM posts")?;
    let mut rows = stmt.query(NO_PARAMS)?;
    let count = rows.next()?.unwrap().get(0)?;

    Ok(count)
}
