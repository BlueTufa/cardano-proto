use crate::args::Args;
use crate::blockfrost::{collect_cover_images, get_assets_by_policy_id};
use crate::rest_api::get_bytes_to_file;
use clap::Parser;
use futures::prelude::*;
use std::error::Error;
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
        Ok(()) => log::debug!("Succeeded"),
        Err(err) => {
            log::error!("An error occurred during the operation: {}", err);
        }
    }
}

async fn validate_and_download_assets(args: &Args) -> Result<(), Box<dyn Error>> {
    // validate that the policy is a valid book.io collection
    let collection = book_io::Collection::validate(&args.policy_id)
        .await
        .expect("Collection is invalid.");
    log::debug!("Book.io collection was found: {:?}", collection);

    // In this example, we only need 10 unique images, so a default page size of 100 is likely not an issue.
    // however, for many applications a paging operation would be needed here.
    let assets_policy_by_id = get_assets_by_policy_id(&args.policy_id).await?;
    log::debug!("Found {} assets for policy id.", assets_policy_by_id.len());

    // The preference would be to use a .map here and return a Vec of cover images.  The syntax
    // wasn't quite working with futures::stream, so I opted to temporarily
    let _ = futures::stream::iter(assets_policy_by_id)
        .for_each(|a| async move {
            let image = collect_cover_images(&a.asset).await.unwrap();

            // this section needs refactored into a 'download' fn
            let ipfs_addr = image.src.chars().skip("ipfs://".len()).collect::<String>();
            let url = format!("https://gateway.pinata.cloud/ipfs/{}", ipfs_addr);
            // TODO: support multiple content types based on image.media_type.  Assume png for now.
            let output_path = Path::new(&args.output_dir)
                .join(Path::new(&ipfs_addr))
                .with_extension("png");
            // TODO: A bug here.  Needs to stop after finding 10 unique covers
            get_bytes_to_file(&url, output_path)
                .await
                .expect("Unable to download file");

            log::debug!("{:?}", image);
        })
        .await;

    Ok(())
}
