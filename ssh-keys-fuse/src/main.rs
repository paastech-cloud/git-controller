mod constants;

use async_trait::async_trait;
use constants::{
    DB_HOST_KEY, DB_NAME_KEY, DB_PASSWORD_KEY, DB_USER_KEY, GIT_AUTH_BIN_PATH_KEY, MOUNTPOINT_KEY,
};
use dotenvy::dotenv;
use fuser::{
    FileAttr, FileType, Filesystem, MountOption, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry,
    Request,
};
use libc::ENOENT;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{Error, PgPool, Row};
use std::ffi::OsStr;
use std::time::{Duration, UNIX_EPOCH};
use std::usize;

const TTL: Duration = Duration::from_secs(1); // 1 second

const DOT_SSH_DIR_ATTR: FileAttr = FileAttr {
    ino: 1,
    size: 0,
    blocks: 0,
    atime: UNIX_EPOCH,
    mtime: UNIX_EPOCH,
    ctime: UNIX_EPOCH,
    crtime: UNIX_EPOCH,
    kind: FileType::Directory,
    perm: 0o600,
    nlink: 2,
    uid: 1000,
    gid: 1000,
    rdev: 0,
    flags: 0,
    blksize: 512,
};

const AUTHORIZED_KEYS_ATTR: FileAttr = FileAttr {
    ino: 2,
    size: 90,
    blocks: 1,
    atime: UNIX_EPOCH, // 1970-01-01 00:00:00
    mtime: UNIX_EPOCH,
    ctime: UNIX_EPOCH,
    crtime: UNIX_EPOCH,
    kind: FileType::RegularFile,
    perm: 0o600,
    nlink: 1,
    uid: 1000,
    gid: 1000,
    rdev: 0,
    flags: 0,
    blksize: 512,
};

#[derive(sqlx::FromRow)]
struct SshKey {
    user_id: String,
    value: String,
}

#[async_trait]
trait HelloFSTrait {
    async fn query_content(&self) -> Result<Vec<SshKey>, Error>;
    async fn get_content(&self) -> String;
    async fn get_content_size(&self) -> usize;
}

struct HelloFS {
    pub git_auth_binary_path: String,
    pub connection_pool: PgPool,
}

#[async_trait]
impl HelloFSTrait for HelloFS {
    async fn query_content(&self) -> Result<Vec<SshKey>, Error> {
        sqlx::query_as::<_, SshKey>("SELECT user_id, value FROM ssh_keys;")
            .fetch_all(&self.connection_pool)
            .await
    }
    async fn get_content(&self) -> String {
        let content: Vec<SshKey> = match self.query_content().await {
            Ok(content) => content,
            Err(_) => Vec::new(),
        };

        let mut result = String::new();
        for row in content {
            result += &format!(
                r#"command="{} {}" {}"#,
                &self.git_auth_binary_path, row.user_id, row.value
            );
            result += "\n";
        }

        result
    }
    async fn get_content_size(&self) -> usize {
        self.get_content().await.len()
    }
}

impl Filesystem for HelloFS {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        if parent == 1 && name.to_str() == Some("authorized_keys") {
            reply.entry(&TTL, &AUTHORIZED_KEYS_ATTR, 0);
        } else {
            reply.error(ENOENT);
        }
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        match ino {
            1 => reply.attr(&TTL, &DOT_SSH_DIR_ATTR),
            2 => reply.attr(&TTL, &AUTHORIZED_KEYS_ATTR),
            _ => reply.error(ENOENT),
        }
    }

    fn read(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        _size: u32,
        _flags: i32,
        _lock: Option<u64>,
        reply: ReplyData,
    ) {
        if ino == 2 {
            reply.data(&self.get_content().await.as_bytes()[offset as usize..]);
        } else {
            reply.error(ENOENT);
        }
    }

    fn readdir(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {
        if ino != 1 {
            reply.error(ENOENT);
            return;
        }

        let entries = vec![
            (1, FileType::Directory, "."),
            (1, FileType::Directory, ".."),
            (2, FileType::RegularFile, "authorized_keys"),
        ];

        for (i, entry) in entries.into_iter().enumerate().skip(offset as usize) {
            if reply.add(entry.0, (i + 1) as i64, entry.1, entry.2) {
                break;
            }
        }

        reply.ok();
    }
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let mountpoint = std::env::var(MOUNTPOINT_KEY).unwrap();

    let connection_url = format!(
        "postgres://{}:{}@{}/{}",
        std::env::var(DB_USER_KEY).unwrap(),
        std::env::var(DB_PASSWORD_KEY).unwrap(),
        std::env::var(DB_HOST_KEY).unwrap(),
        std::env::var(DB_NAME_KEY).unwrap()
    );

    let hello_fs = HelloFS {
        connection_pool: PgPoolOptions::new()
            .max_connections(5)
            .connect(&connection_url)
            .await?,
        git_auth_binary_path: std::env::var(GIT_AUTH_BIN_PATH_KEY).unwrap(),
    };

    let _: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(&hello_fs.connection_pool)
        .await?;

    fuser::mount2(hello_fs, mountpoint, &[MountOption::AutoUnmount]).unwrap();

    Ok(())
}
