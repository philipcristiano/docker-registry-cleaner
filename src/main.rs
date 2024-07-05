use std::error::Error;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let registry_url = "http://your-registry-url/v2";

    // Get list of repositories
    let repos: Vec<String> = get_repositories(&client, registry_url).await?;

    for repo in repos {
        // Get list of tags for the repository
        let tags: Vec<String> = get_tags(&client, registry_url, &repo).await?;

        // Get image manifests and their last-updated labels
        let mut labeled_images: Vec<(String, String)> = Vec::new();
        let mut unlabeled_images: Vec<String> = Vec::new();

        for tag in tags {
            if let Some(last_updated) = get_last_updated_label(&client, registry_url, &repo, &tag).await? {
                labeled_images.push((tag, last_updated));
            } else {
                unlabeled_images.push(tag);
            }
        }

        // If no images have the last-updated label, skip this repository
        if labeled_images.is_empty() {
            println!("Skipping repository {} as no images have the last-updated label", repo);
            continue;
        }

        // Sort labeled images by last-updated label
        labeled_images.sort_by(|a, b| b.1.cmp(&a.1));

        // Keep the latest 3 labeled images, delete the rest
        let images_to_keep: Vec<String> = labeled_images.iter().take(3).map(|(tag, _)| tag.clone()).collect();

        // Delete all images not in the keep list
        delete_images(&client, registry_url, &repo, &images_to_keep, &labeled_images, &unlabeled_images).await?;
    }

    Ok(())
}

async fn delete_images(
    client: &Client,
    registry_url: &str,
    repo: &str,
    images_to_keep: &[String],
    labeled_images: &[(String, String)],
    unlabeled_images: &[String]
) -> Result<(), Box<dyn Error>> {
    // Delete labeled images not in the keep list
    for (tag, _) in labeled_images.iter().skip(3) {
        delete_image(client, registry_url, repo, tag).await?;
        println!("Deleted labeled image {}:{}", repo, tag);
    }

    // Delete all unlabeled images
    for tag in unlabeled_images {
        delete_image(client, registry_url, repo, tag).await?;
        println!("Deleted unlabeled image {}:{}", repo, tag);
    }

    Ok(())
}

async fn get_repositories(client: &Client, registry_url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let url = format!("{}/catalog", registry_url);
    let response: Value = client.get(&url).send().await?.json().await?;
    Ok(response["repositories"].as_array()
        .ok_or("Invalid response format")?
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect())
}

async fn get_tags(client: &Client, registry_url: &str, repo: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let url = format!("{}/{}/tags/list", registry_url, repo);
    let response: Value = client.get(&url).send().await?.json().await?;
    Ok(response["tags"].as_array()
        .ok_or("Invalid response format")?
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect())
}

async fn get_last_updated_label(client: &Client, registry_url: &str, repo: &str, tag: &str) -> Result<Option<String>, Box<dyn Error>> {
    let url = format!("{}/{}/manifests/{}", registry_url, repo, tag);
    let response: Value = client.get(&url)
        .header("Accept", "application/vnd.docker.distribution.manifest.v2+json")
        .send().await?.json().await?;

    let labels = response["config"]["labels"].as_object()
        .ok_or("Invalid response format")?;

    Ok(labels.get("image.last-updated").and_then(|v| v.as_str().map(String::from)))
}

async fn delete_image(client: &Client, registry_url: &str, repo: &str, tag: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/{}/manifests/{}", registry_url, repo, tag);

    // First, get the digest
    let digest = client.get(&url)
        .header("Accept", "application/vnd.docker.distribution.manifest.v2+json")
        .send().await?
        .headers()
        .get("Docker-Content-Digest")
        .ok_or("Digest not found")?
        .to_str()?
        .to_string();

    // Then, delete the image using the digest
    let delete_url = format!("{}/{}/manifests/{}", registry_url, repo, digest);
    client.delete(&delete_url).send().await?;

    Ok(())
}
