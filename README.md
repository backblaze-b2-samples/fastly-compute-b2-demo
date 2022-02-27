# Backblaze B2 / Fastly Compute@Edge Demo

[![Deploy to Fastly](https://deploy.edgecompute.app/button)](https://deploy.edgecompute.app/deploy)

Serve content from one of a set of B2 regions, depending on the edge server in use.

**For more details about this demo, see the blog entry, [Optimize Access to Backblaze B2 with Fastly Compute@Edge](https://link.tbd)**

## Prerequisites

* Backblaze B2 accounts in each of the US and EU regions
* A bucket in each account, each bucket containing the same set of resources

If you are using private buckets, you will also need an application key in each account with at least read access to the bucket.

## Configuration

### Private Buckets

If you are using private buckets (the default), you will need to create an [Edge Dictionary](https://docs.fastly.com/en/guides/about-edge-dictionaries) named `bucket_auth` with the following values for each region:

* `eu_origin_access_key_id` - B2 application key ID for EU bucket
* `eu_origin_secret_access_key` - B2 application key for EU bucket
* `us_origin_access_key_id` - B2 application key ID for US bucket
* `us_origin_secret_access_key` - B2 application key for US bucket

If you use the 'Deploy to Fastly' button you will be prompted for these credentials and the Edge Dictionary will be created as part of the deployment process. Read the configuration prompts carefully as, at present, they are not shown in a consistent order.

Note that the application is unable to access private buckets when run in the local test server (`fastly compute serve`), since Edge Dictionaries are not accessible in this mode.

### Public Buckets

Comment out the last three lines of [Cargo.toml](Cargo.toml) to disable AWS signatures and use public buckets:

```toml
# You may comment out the following lines to skip AWS V4 signing
# if you wish to use public buckets
#[features]
#default = ["auth"]
#auth = ["chrono", "hmac-sha256", "hex", "urlencoding"]
```

### Origin Details

**IMPORTANT**: you must edit the `EU_ORIGIN` and `US_ORIGIN` structs in `src/config.rs` to match your environment:

* `bucket_name`: The name of the bucket in that region, e.g. `my-web-content-eu`
* `bucket_host`: The endpoint of the bucket, e.g. `s3.eu-central-003.backblazeb2.com` for the EU region

Once you have updated `src/config.rs`, re-publish the application, and you should be able to access resources via its Fastly domain.

## Accessing your content

You should be able to access all of the files in the buckets using your Fastly Compute application's domain, which is in the form `<adjective>.<adverb>.<noun>.edgecompute.app`. For example, if your application's domain is `amazingly-awesome-unicorn.edgecompute.app` and you have a file, `index.html` in your bucket, it should be available at:

```
https://amazingly-awesome-unicorn.edgecompute.app/index.html
```

Using curl, or your browser's developer tools, you will see a custom HTTP header, `X-B2-Host`, indicating which origin was used. For a bucket named `my-web-content-us` with B2 endpoint `s3.us-west-001.backblazeb2.com`, `X-B2-Host` will be set to:

```
my-web-content-us.s3.us-west-001.backblazeb2.com
```

You can simulate the request being processed by an edge server in a different Fastly datacenter by specifying the datacenter's three-letter 'pop' code as a query parameter. For example, to simulate the request being processed by the Amsterdam edge server, `AMS`, you would use:

```
https://amazingly-awesome-unicorn.edgecompute.app/index.html?pop=AMS
```

The command `fastly pops` provides details of the Fastly datacenters.

## Understanding the code

This demo is adapted from the [Compute@Edge static content starter kit for Rust](https://github.com/fastly/compute-starter-kit-rust-static-content). When the application receives a request, it performs the following operations:

1. Discovers the three-letter abbreviation for the datacenter ('pop') in which it is running, using the following mechanism:
  * If the requested URL contains a `pop` query parameter, for example, `https://three.interesting.words.edgecompute.app/image.png?pop=AMS` than its value is used as the pop.
  * Otherwise, if the application is running on the local test server rather than the Fastly Compute@Edge environment, use the default set in `src/config.rs`.
  * Otherwise, use the value that Fastly exposes via the `FASTLY_POP` environment variable.
2. Selects the US or EU origin by looking up the pop in the `POP_ORIGIN` map defined in `src/config.rs`.
3. Removes query parameters, so that they do not interfere with Fastly caching resources.
4. Sets the request `Host` header according to the selected origin.
5. Applies an AWS V4 signature to the request.
6. Forwards the request to the origin's Fastly backend and returns the response.

Feel free to fork the project and adapt the code to your requirements!