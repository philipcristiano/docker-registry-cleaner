use clap::Parser;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;
use std::string::ToString;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, value_enum, default_value = "DEBUG")]
    log_level: tracing::Level,
    #[arg(long)]
    registry: String,
    #[arg(long)]
    last_updated_label: String,
    #[arg(long, action)]
    dry_run: bool,
    #[arg(long, default_value = "5")]
    keep_n: usize,
}
use tracing::Level;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let delete_with = Delete::new(args.dry_run);
    service_conventions::tracing::setup(args.log_level);
    let client = Client::new();
    let registry_url = &format!("{}/v2", &args.registry);

    // Get list of repositories
    let repos: Vec<String> = get_repositories(&client, registry_url).await?;
    //let repos = vec!("philipcristiano/et");

    tracing::debug!("Repos {repos:?}");

    for repo in repos {
        // Get list of tags for the repository
        if let Ok(tags) = get_tags(&client, registry_url, &repo).await {
            // Get image manifests and their last-updated labels
            let mut labeled_images: Vec<(String, String)> = Vec::new();
            let mut unlabeled_images: Vec<String> = Vec::new();

            for tag in tags {
                match get_last_updated_label(
                    &client,
                    registry_url,
                    &repo,
                    &tag,
                    &args.last_updated_label,
                )
                .await
                {
                    Ok(Some(last_updated)) => labeled_images.push((tag, last_updated)),
                    Ok(None) => unlabeled_images.push(tag),
                    Err(e) => tracing::error!(error = e, repo = repo, tag = tag, "Error"),
                }
            }

            // If no images have the last-updated label, skip this repository
            if labeled_images.is_empty() {
                tracing::info!(
                    "Skipping repository {} as no images have the last-updated label",
                    repo
                );
                continue;
            }

            // Sort labeled images by last-updated label
            labeled_images.sort_by(|a, b| b.1.cmp(&a.1));

            // Keep the latest 3 labeled images, delete the rest
            let images_to_keep: Vec<String> = labeled_images
                .iter()
                .take(args.keep_n)
                .map(|(tag, _)| tag.clone())
                .collect();

            // Delete all images not in the keep list
            delete_images(
                &delete_with,
                &client,
                registry_url,
                &repo,
                &labeled_images,
                &unlabeled_images,
            )
            .await?;
        } else {
            tracing::info!(repo = repo, "Could not get tags")
        }
    }

    Ok(())
}

enum DeleteType {
    Real,
    NoOp,
}

struct Delete {
    delete_type: DeleteType,
}

impl Delete {
    fn new(dry_run: bool) -> Delete {
        let delete_type = match dry_run {
            true => DeleteType::NoOp,
            false => DeleteType::Real,
        };
        Delete { delete_type }
    }

    async fn delete_image(
        &self,
        client: &Client,
        registry_url: &str,
        repo: &str,
        tag: &str,
    ) -> Result<(), Box<dyn Error>> {
        match self.delete_type {
            DeleteType::NoOp => {
                tracing::info!(repo = repo, tag = tag, "Dry run, not deleteing");

                Ok(())
            }
            DeleteType::Real => {
                let url = format!("{}/{}/manifests/{}", registry_url, repo, tag);

                // First, get the digest
                let digest = client
                    .get(&url)
                    .header(
                        "Accept",
                        "application/vnd.docker.distribution.manifest.v2+json",
                    )
                    .send()
                    .await?
                    .headers()
                    .get("Docker-Content-Digest")
                    .ok_or("Digest not found")?
                    .to_str()?
                    .to_string();

                // Then, delete the image using the digest
                let delete_url = format!("{}/{}/manifests/{}", registry_url, repo, digest);
                tracing::debug!("Delete {url}");
                client.delete(&url).send().await?;
                tracing::debug!("Delete {delete_url}");
                client.delete(&delete_url).send().await?;

                Ok(())
            }
        }
    }
}

async fn delete_images(
    delete_with: &Delete,
    client: &Client,
    registry_url: &str,
    repo: &str,
    labeled_images: &[(String, String)],
    unlabeled_images: &[String],
) -> Result<(), Box<dyn Error>> {
    // Delete labeled images not in the keep list
    for (tag, _) in labeled_images.iter().skip(3) {
        tracing::info!(repository=repo, tag=tag, "Labeled image elligible for deletion");
        delete_with
            .delete_image(client, registry_url, repo, tag)
            .await?;
    }

    // Delete all unlabeled images
    for tag in unlabeled_images {
        tracing::info!(repository=repo, tag=tag, "Unlaballed image elligible for deletion");
        delete_with
            .delete_image(client, registry_url, repo, tag)
            .await?;
    }

    Ok(())
}

#[derive(serde::Deserialize, Clone, Debug)]
struct CatalogResponse {
    repositories: Vec<String>,
}

async fn get_repositories(
    client: &Client,
    registry_url: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let url = format!("{}/_catalog", registry_url);
    tracing::debug!("URL {url}");
    let response = client.get(&url).send().await?;
    let catalog = response.json::<CatalogResponse>().await?;
    Ok(catalog.repositories.to_owned())
}

#[derive(serde::Deserialize, Clone, Debug)]
struct TagsResponse {
    tags: Vec<String>,
}

async fn get_tags(
    client: &Client,
    registry_url: &str,
    repo: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let url = format!("{}/{}/tags/list", registry_url, repo);
    tracing::debug!("URL {url}");
    let response = client.get(&url).send().await?;
    let tag_resp = response.json::<TagsResponse>().await?;
    Ok(tag_resp.tags.to_owned())
}

#[derive(serde::Deserialize, Clone, Debug)]
struct ManifestResponse {
    manifests: Vec<ManifestDoc>,
}
#[derive(serde::Deserialize, Clone, Debug)]
struct ManifestDoc {
    digest: String,
    annotations: Option<Annotations>,
}
#[derive(serde::Deserialize, Clone, Debug)]
struct ChildManifestDoc {
    config: ManifestConfig,
}
#[derive(serde::Deserialize, Clone, Debug)]
struct ManifestConfig {
    digest: String,
}

#[derive(serde::Deserialize, Clone, Debug)]
struct Annotations {
    #[serde(rename = "vnd.docker.reference.digest")]
    reference_digest: Option<String>,
}

#[derive(serde::Deserialize, Clone, Debug)]
struct ManifestBlob {
    config: HashMap<String, HashMap<String, String>>,
}

async fn get_last_updated_label(
    client: &Client,
    registry_url: &str,
    repo: &str,
    tag: &str,
    last_updated_label: &str,
) -> Result<Option<String>, Box<dyn Error>> {
    let url = format!("{}/{}/manifests/{}", registry_url, repo, tag);
    tracing::debug!("URL {url}");
    let response = client
        .get(&url)
        .header("Accept", "application/vnd.oci.image.manifest.v1+json")
        .send()
        .await?
        .json::<ManifestResponse>()
        .await?;
    for manifest in response.manifests {
        if let Some(annotation) = manifest.annotations {
            if let None = annotation.reference_digest {
                continue;
            }
            let url = format!("{}/{}/manifests/{}", registry_url, repo, manifest.digest);
            tracing::debug!("Child manifest URL {url}");
            let response = client
                .get(&url)
                .header("Accept", "application/vnd.oci.image.manifest.v1+json")
                .send()
                .await?
                .json::<ChildManifestDoc>()
                .await?;
            tracing::debug!("Reference Response {response:?}");

            let blob_digest = response.config.digest;
            let url = format!("{}/{}/blobs/{}", registry_url, repo, blob_digest);
            tracing::debug!("Blob URL {url}");

            let response = client
                .get(&url)
                .header("Accept", "application/vnd.oci.image.manifest.v1+json")
                .send()
                .await?
                .text()
                .await?;
            tracing::debug!("Blob Response txt {response:?}");
            let response = client
                .get(&url)
                .header("Accept", "application/vnd.oci.image.manifest.v1+json")
                .send()
                .await?
                .json::<ManifestBlob>()
                .await?;
            tracing::debug!("Blob Response {response:?}");
            if let Some(labels) = response.config.get("Labels") {
                if let Some(last_updated) = labels.get(last_updated_label) {
                    return Ok(Some(last_updated.to_owned()));
                }
            }
        } else {
            // No annotations, skip this manifest reference
            continue;
        }
    }
    //println!("Response {response:?}");
    //let labels = response["config"]["labels"].as_object()
    //    .ok_or("2 Invalid response format")?;

    //Ok(labels.get("image.last-updated").and_then(|v| v.as_str().map(String::from)))
    Ok(None)
}
