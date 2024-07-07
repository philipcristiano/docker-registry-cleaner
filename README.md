# docker-registry-cleaner
Clean old images from your Docker registry

[Docker Image](https://hub.docker.com/r/philipcristiano/docker-registry-cleaner)

## Usage

```
Usage: docker-registry-cleaner [OPTIONS] --registry <REGISTRY> --last-updated-label <LAST_UPDATED_LABEL>

Options:
  -l, --log-level <LOG_LEVEL>                    [default: DEBUG]
      --registry <REGISTRY>
      --last-updated-label <LAST_UPDATED_LABEL>
      --keep-n <KEEP_N>                          [default: 5]
  -h, --help                                     Print help

```

Keeps last `<KEEP_N>` images by finding all images with `<LAST_UPDATED_LABEL>`, sorting the value of the label, and removing other tags.

* If there are no images with that label then no tags will be removed.
* Only works for image manifests of `application/vnd.oci.image.manifest.v1+json`
* Other manifest types are skipped
