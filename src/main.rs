use crate::args::Args;
use crate::blockfrost::{collect_cover_images, get_assets_by_policy_id, File};
use crate::rest_api::get_bytes_to_file;
use clap::Parser;
use fs::create_dir_all;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;

mod args;
mod blockfrost;
mod book_io;
mod rest_api;

#[tokio::main]
async fn main() {
    env_logger::init();
    log::debug!("CLI starting...");

    let args = Args::parse();
    log::debug!("Command line initialized with the following args: {:?}", args);

    match validate_and_download_assets(&args).await {
        Ok(()) => log::info!("Succeeded."),
        Err(err) => {
            log::error!("An error occurred during the operation: {}", err);
        }
    }
}

async fn validate_and_download_assets(args: &Args) -> Result<(), Box<dyn Error>> {
    // validate that the policy is a valid book.io collection
    let collection = book_io::Collection::validate(&args.policy_id).await;
    if let Err(coll) = collection {
        return Err(Box::try_from(coll).unwrap());
    }
    log::debug!("Book.io collection was found: {:?}", collection);

    // In this example, we only need 10 unique images, so a default page size of 100 is likely not an issue.
    // however, for many applications a paging operation would be needed here.
    let assets_policy_by_id = get_assets_by_policy_id(&args.policy_id).await?;
    log::debug!("Found {} assets for policy id.", assets_policy_by_id.len());

    let mut downloaded_set: HashSet<String> = HashSet::new();
    // The preference would be to use a .map here and return a Vec of cover images.  The syntax
    // wasn't quite working with futures::stream, so this is a temporary solution.
    for asset in assets_policy_by_id {
        match collect_cover_images(&asset.asset).await {
            Ok(image) => {
                if !downloaded_set.contains(&image.src) {
                    download(&args, &image).await.expect("Unable to download file.");
                    log::debug!("Downloaded {}", &image.src);
                    downloaded_set.insert(image.src);
                }
            }
            Err(msg) => log::warn!("The asset does not contain high-res image metadata.  {}", msg),
        }
        if downloaded_set.len() == 10 {
            break;
        }
    }
    Ok(())
}

async fn download(args: &Args, file: &File) -> Result<(), Box<dyn Error>> {
    let ipfs_addr = file.src.chars().skip("ipfs://".len()).collect::<String>();
    // I couldn't get a token for bitfrost IPFS, so I'm using Pinata as a quick workaround.
    let url = format!("https://gateway.pinata.cloud/ipfs/{}", ipfs_addr);
    // create the directory if it does not exist.
    create_dir_all(Path::new(&args.output_dir)).expect("Unable to create output directory.");
    // TODO: support multiple content types based on image.media_type.  Assume png for now.
    let output_path = Path::new(&args.output_dir)
        .join(Path::new(&ipfs_addr))
        .with_extension("png");
    if let Err(err) = get_bytes_to_file(&url, output_path).await {
        log::warn!("Unable to download file.  Reason: {}", err);
    }
    Ok(())
}
