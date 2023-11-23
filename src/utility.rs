use std::path::Path;
use futures::TryStreamExt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn download_file<P: AsRef<Path>>(url: &str, dst_path: P) -> anyhow::Result<()> {
	let dst_path = dst_path.as_ref();

	if dst_path.exists() {
		log::info!("{} already exists, skipping download", dst_path.display());
		return Ok(());
	}

	let response = reqwest::get(url).await?;

	// Ensure the response status is success
	if response.status().is_success() {
		let mut dst_file = File::create(dst_path).await?;
		let mut stream = response.bytes_stream();
		let mut bytes_dowloaded = 0;
		let mut last_reported = 0;

		// Write chunks to file as they are received
		while let Some(chunk) = stream.try_next().await? {
			bytes_dowloaded += chunk.len();
			let since_last_report = bytes_dowloaded - last_reported;
			if since_last_report > 1_000_000 {
				log::info!("[{}] {} bytes downloaded", url, bytes_dowloaded);
				last_reported = bytes_dowloaded;
			}
			dst_file.write_all(&chunk).await?;
		}

		log::info!("Downloaded {} bytes to {}", bytes_dowloaded, dst_path.display());
	} else {
		log::error!("Failed to download file. Status: {}", response.status());
		return Err(anyhow::anyhow!("Failed to download file"));
	}

	Ok(())
}

pub fn decompress_file<P: AsRef<Path>>(src_path: P, dst_path: P) -> anyhow::Result<()> {
	let src_path = src_path.as_ref();
	let dst_path = dst_path.as_ref();

	if dst_path.exists() {
		log::info!("{} already exists, skipping decompression", dst_path.display());
		return Ok(());
	}

	let src_file = std::fs::File::open(src_path)?;
	let mut dst_file = std::fs::File::create(dst_path)?;
	let mut decoder = flate2::read::GzDecoder::new(src_file);
	std::io::copy(&mut decoder, &mut dst_file)?;
	Ok(())
}