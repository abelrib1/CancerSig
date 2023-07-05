use reqwest::Error;
use std::collections::HashMap;
use tokio::io::AsyncWriteExt;
use serde_json::Value;
use async_fs::File;
use csv::ReaderBuilder;
use sysinfo::{DiskExt, System, SystemExt};

pub async fn download_gdc_data(manifest_path: &str) -> Result<(), Error> {
    let system = sysinfo::System::new_all();
    let all_disks = system.get_disks();
    let root_disk = all_disks.iter().find(|x| x.get_mount_point() == "/").unwrap();

    let mut rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(manifest_path).unwrap();

    for result in rdr.deserialize() {
        let record: HashMap<String, String> = result.unwrap();
        let id = &record["id"];
        
        if root_disk.get_available_space() < record["size"].parse::<u64>().unwrap() {
            println!("Insufficient disk space. Download stopped.");
            break;
        }

        let file_data = reqwest::get(&format!("https://api.gdc.cancer.gov/data/{}", id)).await?.bytes().await?;
        let metadata = reqwest::get(&format!("https://api.gdc.cancer.gov/files/{}?format=json&pretty=true", id)).await?.json::<Value>().await?;

        let mut file = File::create(format!("data/{}", &record["filename"])).await?;
        file.write_all(&file_data).await?;

        let mut metadata_file = File::create("metadata.json").await?;
        metadata_file.write_all(serde_json::to_string_pretty(&metadata)?.as_bytes()).await?;
    }

    Ok(())
}
