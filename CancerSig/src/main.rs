use clap::{Arg, App, SubCommand};
use std::collections::HashMap;
use tokio::io::AsyncWriteExt;
use tokio::fs::File;
use tokio::runtime::Builder;
use serde_json::Value;
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
    let sem = Arc::new(Semaphore::new(40));
    let matches = App::new("CancerSig")
        .version("1.0")
        .about("Downloads data from the GDC")
        .subcommand(
            SubCommand::with_name("download")
                .about("Downloads data using a manifest file")
                .arg(Arg::with_name("manifest")
                    .help("The path to the manifest file")
                    .required(true)
                    .index(1)))
        .get_matches();

        if let Some(matches) = matches.subcommand_matches("download") {
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

                    tasks.push(async move {
                        let _permit = sem_clone.acquire().await;
                        download_gdc_data(id, filename).await; 
                    });
                }
                while let Some(()) = tasks.next().await { /* keep polling until None */ }
                // while let Some(task) = tasks.next().await {
                //     match task.await {
                //         Ok(_) => println!("Task completed successfully"), 
                //         Err(e) => eprintln!("Task panicked with error: {:?}", e),
                //     }  
                // }
            }
        }
    
        Ok(())
    }

async fn download_gdc_data(id: String, filename: String) {
    match _download_gdc_data(id.clone(), filename.clone()).await {
        Ok(_) => println!("Finished downloading file {}", filename),
        Err(err) => eprintln!("Error downloading file {} with id {}: {}", filename, id, err),
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