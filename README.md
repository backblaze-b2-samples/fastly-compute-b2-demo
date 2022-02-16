# Backblaze B2 / Fastly Compute@Edge Demo

[![Deploy to Fastly](https://deploy.edgecompute.app/button)](https://deploy.edgecompute.app/deploy)

Serve content from multiple B2 regions depending on the edge server in use.

**For more details about this demo, see the blog entry, [Optimize Access to Backblaze B2 with Fastly Compute@Edge](https://link.tbd)**

## Prerequisites

* Backblaze B2 accounts in each of the US and EU regions
* A bucket in each account, each containing the same set of resources
* An application key in each account with at least read access to the bucket

## Configuration

Edit the `EU_ORIGIN` and `US_ORIGIN` structs in `src/config.rs` to match your environment:

* `backend_name`: The name of your storage backend in the Fastly UI
* `bucket_name`: The name of the bucket
* `bucket_host`: The endpoint of the bucket, e.g. `s3.eu-central-003.backblazeb2.com`

If you are using a private bucket, you will need to create an [edge dictionary](https://docs.fastly.com/en/guides/about-edge-dictionaries) named `bucket_auth` with the following values for each region:

 * `<backend_name>_access_key_id` - B2 application key ID
 * `<backend_name>_secret_access_key` - B2 application key

## Understanding the code

This demo is based on the [Compute@Edge static content starter kit for Rust](https://github.com/fastly/compute-starter-kit-rust-static-content). The application's main function is very simple; it 


## Security issues

Please see our [SECURITY.md](SECURITY.md) for guidance on reporting security-related issues.
