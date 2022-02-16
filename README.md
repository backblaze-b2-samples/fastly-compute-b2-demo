# Backblaze B2 / Fastly Compute@Edge Demo

[![Deploy to Fastly](https://deploy.edgecompute.app/button)](https://deploy.edgecompute.app/deploy)

Serve content from one of a set of B2 regions, depending on the edge server in use.

**For more details about this demo, see the blog entry, [Optimize Access to Backblaze B2 with Fastly Compute@Edge](https://link.tbd)**

## Prerequisites

* Backblaze B2 accounts in each of the US and EU regions
* A bucket in each account, each containing the same set of resources
* An application key in each account with at least read access to the bucket

## Configuration

Edit the `EU_ORIGIN` and `US_ORIGIN` structs in `src/config.rs` to match your environment:

* `backend_name`: The name of your storage backend, as configured in the Fastly UI
* `bucket_name`: The name of the bucket
* `endpoint`: The endpoint of the bucket, e.g. `s3.eu-central-003.backblazeb2.com`

If you are using a private bucket, you will need to create an [Edge Dictionary](https://docs.fastly.com/en/guides/about-edge-dictionaries) named `bucket_auth` with the following values for each region:

* `<backend_name>_access_key_id` - B2 application key ID
* `<backend_name>_secret_access_key` - B2 application key

Note that the application is unable to access private buckets when run in the local test server (`fastly compute serve`), since Edge Dictionaries are not accessible.

## Understanding the code

This demo is adapted from the [Compute@Edge static content starter kit for Rust](https://github.com/fastly/compute-starter-kit-rust-static-content). When the application receives a request, it performs the following operations:

1. Discovers the three-letter abbreviation for the edge server ('pop') on which it is running, using the following mechanism:
  * If the requested URL contains a `pop` query parameter, for example, `https://three.interesting.words.edgecompute.app/image.png?pop=AMS` than its value is used as the pop.
  * Otherwise, if the application is running on the local test server rather than the Fastly Compute@Edge environment, use the default set in `src/config.rs`.
  * Otherwise, use the value that Fastly exposes via the `FASTLY_POP` environment variable.
2. Selects the US or EU origin by looking up the pop in the `POP_ORIGIN` map defined in `src/config.rs`.
3. Removes query parameters, so that they do not interfere with Fastly caching resources.
4. Sets the request `Host` header according to the selected origin.
5. Applies an AWS V4 signature to the request.
6. Forwards the request to the origin's Fastly backend and returns the response.