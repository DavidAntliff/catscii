// https://fasterthanli.me/series/building-a-rust-service-with-nix/part-3

use pretty_hex::PrettyHex;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    // let url = get_cat_image_url().await.unwrap();
    // println!("The image is at {}", url);
    // let image_bytes = get_cat_image_bytes().await.unwrap();
    // println!("{:?}", &image_bytes[..200].hex_dump());
    let art = get_cat_ascii_art().await.unwrap();
    println!("{art}");
}

async fn get_cat_ascii_art() -> color_eyre::Result<String> {
    #[derive(Deserialize)]
    struct CatImage {
        url: String,
    }

    let api_url = "https://api.thecatapi.com/v1/images/search";
    let client = reqwest::Client::default();

    let image = client
        .get(api_url)
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<CatImage>>()
        .await?
        .pop()
        .ok_or_else(|| color_eyre::eyre::eyre!("The Cat API returned no images"))?;

    let image_bytes = client
        .get(image.url)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;

    let image = image::load_from_memory(&image_bytes)?;
    let ascii_art = artem::convert(image, artem::options::OptionBuilder::new().build());

    Ok(ascii_art)
}

async fn get_cat_image_bytes() -> color_eyre::Result<Vec<u8>> {
    #[derive(Deserialize)]
    struct CatImage {
        url: String,
    }

    let api_url = "https://api.thecatapi.com/v1/images/search";
    let client = reqwest::Client::default();

    let image = client
        .get(api_url)
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<CatImage>>()
        .await?
        .pop()
        .ok_or_else(|| color_eyre::eyre::eyre!("The Cat API returned no images"))?;

    Ok(client
        .get(image.url)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?
        .to_vec())
}

async fn get_cat_image_url() -> color_eyre::Result<String> {
    let api_url = "https://api.thecatapi.com/v1/images/search";
    let res = reqwest::get(api_url).await?;
    if !res.status().is_success() {
        return Err(color_eyre::eyre::eyre!(
            "The Cat API returned HTTP {}",
            res.status(),
        ));
    }

    #[derive(Deserialize)]
    struct CatImage {
        //id: String,
        url: String,
        // width: usize,
        // height: usize,
    }

    let mut images: Vec<CatImage> = res.json().await?;
    let Some(image) = images.pop() else {
        return Err(color_eyre::eyre::eyre!("The Cat API returned no images"))
    };
    Ok(image.url)
}
