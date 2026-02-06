mod fixtures;
mod utils;

use async_zip::{tokio::write::ZipFileWriter, Compression, ZipEntryBuilder};
use fixtures::{server, Error, TestServer};
use rstest::rstest;
use sha2::{Digest, Sha256};
use std::path::Path;
use tokio::runtime::Runtime;

fn write_zip(path: &Path, entries: Vec<(&str, &[u8])>) -> Result<(), Error> {
    let rt = Runtime::new()?;
    rt.block_on(async {
        let file = tokio::fs::File::create(path).await?;
        let mut writer = ZipFileWriter::with_tokio(file);
        for (name, data) in entries {
            let builder = ZipEntryBuilder::new(name.into(), Compression::Stored);
            writer.write_entry_whole(builder, data).await?;
        }
        writer.close().await?;
        Ok::<(), anyhow::Error>(())
    })?;
    Ok(())
}

#[rstest]
fn zip_browse_lists_entries(
    #[with(&["--allow-zip-browse"])] server: TestServer,
) -> Result<(), Error> {
    let zip_path = server.path().join("archive.zip");
    write_zip(&zip_path, vec![("folder/note.txt", b"note"), ("root.txt", b"root")])?;

    assert!(zip_path.exists());

    let url = format!("{}archive.zip/?json", server.url());
    let resp = reqwest::blocking::get(url)?;
    assert_eq!(resp.status(), 200);
    let body = resp.text()?;
    let value: serde_json::Value = serde_json::from_str(&body)?;
    let names: Vec<String> = value
        .get("paths")
        .and_then(|v| v.as_array())
        .unwrap()
        .iter()
        .filter_map(|v| v.get("name").and_then(|n| n.as_str()).map(|n| n.to_string()))
        .collect();

    assert!(names.contains(&"folder".to_string()));
    assert!(names.contains(&"root.txt".to_string()));
    Ok(())
}

#[rstest]
fn zip_view_shows_editor_ui(
    #[with(&["--allow-zip-browse"])] server: TestServer,
) -> Result<(), Error> {
    let zip_path = server.path().join("archive.zip");
    write_zip(&zip_path, vec![("folder/note.txt", b"hello")])?;

    let url = format!("{}archive.zip/folder/note.txt?view", server.url());
    let resp = reqwest::blocking::get(url)?;
    assert_eq!(resp.status(), 200);
    let body = resp.text()?;
    let editable = utils::retrieve_edit_file(&body).unwrap_or(false);
    assert!(editable);
    Ok(())
}

#[rstest]
fn zip_hash_works(
    #[with(&["--allow-zip-browse", "--allow-hash"])] server: TestServer,
) -> Result<(), Error> {
    let data = b"hash-me";
    let zip_path = server.path().join("archive.zip");
    write_zip(&zip_path, vec![("folder/note.txt", data)])?;

    let url = format!("{}archive.zip/folder/note.txt?hash", server.url());
    let resp = reqwest::blocking::get(url)?;
    assert_eq!(resp.status(), 200);
    let body = resp.text()?;

    let mut hasher = Sha256::new();
    hasher.update(data);
    let expected = format!("{:x}", hasher.finalize());
    assert_eq!(body, expected);
    Ok(())
}

#[rstest]
fn zip_extensions_allow_custom_formats(
    #[with(&["--allow-zip-browse", "--zip-extensions", "zip,tpf"])] server: TestServer,
) -> Result<(), Error> {
    let zip_path = server.path().join("archive.tpf");
    write_zip(&zip_path, vec![("folder/note.txt", b"note")])?;

    let url = format!("{}archive.tpf/?json", server.url());
    let resp = reqwest::blocking::get(url)?;
    assert_eq!(resp.status(), 200);
    let body = resp.text()?;
    let value: serde_json::Value = serde_json::from_str(&body)?;
    let names: Vec<String> = value
        .get("paths")
        .and_then(|v| v.as_array())
        .unwrap()
        .iter()
        .filter_map(|v| v.get("name").and_then(|n| n.as_str()).map(|n| n.to_string()))
        .collect();

    assert!(names.contains(&"folder".to_string()));
    Ok(())
}

#[rstest]
fn zip_search_filters_entries(
    #[with(&["--allow-zip-browse", "--allow-search"])] server: TestServer,
) -> Result<(), Error> {
    let zip_path = server.path().join("archive.zip");
    write_zip(
        &zip_path,
        vec![
            ("folder/note.txt", b"note"),
            ("folder/readme.md", b"readme"),
            ("root.txt", b"root"),
            ("data.json", b"{}"),
        ],
    )?;

    let url = format!("{}archive.zip/?q=note", server.url());
    let resp = reqwest::blocking::get(url)?;
    assert_eq!(resp.status(), 200);
    let body = resp.text()?;
    let paths = utils::retrieve_index_paths(&body);

    // Search should find "note.txt" but not "readme.md", "root.txt", or "data.json"
    assert!(paths.iter().any(|p| p.contains("note.txt")));
    assert!(!paths.iter().any(|p| p.contains("readme.md")));
    assert!(!paths.iter().any(|p| p.contains("root.txt")));
    assert!(!paths.iter().any(|p| p.contains("data.json")));
    Ok(())
}

#[rstest]
fn zip_edit_shows_editor_ui(
    #[with(&["--allow-zip-browse"])] server: TestServer,
) -> Result<(), Error> {
    let zip_path = server.path().join("archive.zip");
    write_zip(&zip_path, vec![("folder/note.txt", b"hello world")])?;

    let url = format!("{}archive.zip/folder/note.txt?edit", server.url());
    let resp = reqwest::blocking::get(url)?;
    assert_eq!(resp.status(), 200);
    let body = resp.text()?;
    let editable = utils::retrieve_edit_file(&body).unwrap_or(false);
    assert!(editable);
    Ok(())
}
