use async_fuse::{
    FileAttr, FileType, Filesystem, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, Request,
};
use async_trait::async_trait;
use libc::ENOENT;
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;
use sqlx::{Error, Pool, Postgres};
use std::ffi::{OsStr, OsString};
use std::time::{Duration, UNIX_EPOCH};

// Constants
//
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
    perm: 0o755,
    nlink: 2,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
};

// Struct `SSHKey` and `PostgresFS`
//
#[derive(FromRow, Debug)]
pub struct SSHKey {
    user_id: String,
    value: String,
}

pub struct PostgresFS {
    // git_auth_binary_path: String,
    connection: Pool<Postgres>,
}

// PostgresFS implementation
//
impl PostgresFS {
    pub async fn new(url_connection_string: String) -> Result<Self, Error> {
        let connection = PgPoolOptions::new()
            .max_connections(5)
            .connect(url_connection_string.as_str())
            .await?;

        println!("Successfully connected to the database.");
        Ok(PostgresFS { connection })
    }

    async fn get_authorized_keys_attr(&self) -> FileAttr {
        let authorized_keys_length = self.get_length_authorized_keys().await.unwrap();

        FileAttr {
            ino: 2,
            size: authorized_keys_length as u64,
            blocks: 1,
            atime: UNIX_EPOCH,
            mtime: UNIX_EPOCH,
            ctime: UNIX_EPOCH,
            crtime: UNIX_EPOCH,
            kind: FileType::RegularFile,
            perm: 0o644,
            nlink: 1,
            uid: 501,
            gid: 20,
            rdev: 0,
            flags: 0,
        }
    }

    async fn get_content_authorized_keys(&self) -> Result<String, sqlx::Error> {
        let query_result = sqlx::query_as::<Postgres, SSHKey>("SELECT * FROM ssh_keys")
            .fetch_all(&self.connection)
            .await?;

        let formatted = query_result
            .iter()
            .map(|row| format!("{} {}\n", row.user_id, row.value))
            .collect::<String>();

        Ok(formatted)
    }

    async fn get_length_authorized_keys(&self) -> Result<usize, sqlx::Error> {
        let query_result = sqlx::query_as::<Postgres, SSHKey>("SELECT * FROM ssh_keys")
            .fetch_all(&self.connection)
            .await?;

        let formatted = query_result
            .iter()
            .map(|row| format!("{} {}\n", row.user_id, row.value))
            .collect::<String>();

        Ok(formatted.len())
    }

    pub fn mount(filesystem: PostgresFS, mountpoint: String) {
        let options = ["-o", "fsname=postgres_fs,auto_unmount,ro"]
            .iter()
            .map(|o| OsString::from(&o))
            .collect::<Vec<OsString>>();

        if let Err(error) = async_fuse::mount(filesystem, mountpoint, &options) {
            eprintln!("Error mounting file system: {}", error);
        }
        println!("Successfully postgresFS mounted.")
    }
}

// filesystem (fuse) implementation for PostgresFS
//
#[async_trait]
impl Filesystem for PostgresFS {
    async fn lookup(&self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        let authorized_keys_attr = self.get_authorized_keys_attr().await;

        if parent == 1 && name.to_str() == Some("authorized_keys") {
            reply.entry(&TTL, &authorized_keys_attr, 0);
        } else {
            reply.error(ENOENT);
        }
    }

    async fn getattr(&self, _req: &Request, ino: u64, reply: ReplyAttr) {
        let authorized_keys_attr = self.get_authorized_keys_attr().await;

        match ino {
            1 => reply.attr(&TTL, &DOT_SSH_DIR_ATTR),
            2 => reply.attr(&TTL, &authorized_keys_attr),
            _ => reply.error(ENOENT),
        }
    }

    async fn read(
        &self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        _size: u32,
        reply: ReplyData,
    ) {
        if ino == 2 {
            let authorized_keys_content: String = self.get_content_authorized_keys().await.unwrap();
            reply.data(&authorized_keys_content.as_bytes()[offset as usize..]);
        } else {
            reply.error(ENOENT);
        }
    }

    async fn readdir(
        &self,
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
