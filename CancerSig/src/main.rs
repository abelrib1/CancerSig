use clap::{Arg, App, SubCommand};
use std::collections::HashMap;
use tokio::io::AsyncWriteExt;
use tokio::fs::File;
use tokio::runtime::Builder;
use serde_json::{Value, from_str};
use csv::ReaderBuilder;
use std::fs;
use std::io::Write;
use anyhow::Result;
use anyhow::Error;
use std::fs::OpenOptions;
use futures::stream::FuturesUnordered;
use futures::stream::StreamExt;
use tokio::sync::Semaphore;
use tokio::sync::SemaphorePermit;
use std::sync::Arc;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let sem = Arc::new(Semaphore::new(50));
    let matches = App::new("CancerSig")
        .version("1.0")
        .about("Downloads data from the GDC")
        .subcommand(
            SubCommand::with_name("download")
                .about("Downloads data using a manifest file")
                .arg(Arg::with_name("manifest")
                    .help("The path to the manifest file")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("metadata")
                    .long("complete")
                    .value_name("FILE")
                    .help("The path to the metadata file")
                    .takes_value(true)))
        .get_matches();

    let mut processed_files: HashMap<String, Value> = HashMap::new();
    if let Some(matches) = matches.subcommand_matches("download") {
        if let Some(metadata_path) = matches.value_of("metadata") {
            processed_files = load_metadata(metadata_path)?;
        }
        if let Some(manifest_path) = matches.value_of("manifest") {
            
            let mut tasks = FuturesUnordered::new();

            // Parsing CSV and creating async tasks for each record
            let mut rdr = csv::ReaderBuilder::new()
                .delimiter(b'\t')
                .from_path(manifest_path)?;
            for result in rdr.deserialize() {
                let record: HashMap<String, String> = result?;
                let id = record["id"].clone();
                let filename = record["filename"].clone();
                let sem_clone = Arc::clone(&sem);

                if processed_files.get(&id).is_some() {
                    println!("File {} is already processed. Skipping.", filename);
                    continue;
                }
                tasks.push(async move {
                    let _permit = sem_clone.acquire().await;
                    let filename_clone = filename.clone();
                    download_gdc_data(id.clone(), filename_clone).await
                });                
                
            }
            while let Some(result) = tasks.next().await {
                if let Err(e) = result {
                    eprintln!("A task encountered an error: {}", e);
                }
            }     
        }
    }

    Ok(())
}

fn load_metadata(path: &str) -> Result<HashMap<String, Value>, Error> {
    use std::io::BufReader;
    use std::io::prelude::*;
    use std::fs::File;

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut data: HashMap<String, Value> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let value: Value = serde_json::from_str(&line)?;
        let id = value["data"]["file_id"].as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid format: couldn't find 'file_id' in the metadata line"))?;
        data.insert(id.to_string(), value);
    }
    Ok(data)
}

async fn download_gdc_data(id: String, filename: String) -> Result<(), Error> {
    match _download_gdc_data(id.clone(), filename.clone()).await {
        Ok(_) => {
            println!("Finished downloading file {}", filename);
            Ok(())
        },
        Err(e) => {
            eprintln!("Failed to download file {} with error: {}", filename, e);
            Err(e)
        }
    }
}

async fn _download_gdc_data(id: String, filename: String) -> Result<(), anyhow::Error> {
    println!("Downloading file {}", filename);
    let metadata_url = format!("https://api.gdc.cancer.gov/files/{}?format=json&pretty=true", id);
    let data_url = format!("https://api.gdc.cancer.gov/data/{}", id);

    // Fetch and process metadata
    let metadata = reqwest::get(&metadata_url).await?.json::<Value>().await?;


    // Fetch and save file data
    let file_data = reqwest::get(&data_url).await?.bytes().await?;
    let file_path = format!("data/{}", filename);
    fs::create_dir_all("data")?;
    
    let mut file = File::create(&file_path).await?;
    file.write_all(&file_data).await?;


    let mut metadata_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("metadata.json")?;
    writeln!(metadata_file, "{}", serde_json::to_string(&metadata)?)?;

    Ok(())
}