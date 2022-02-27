use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

/// Default edge server code - used when running on the local test server
pub(crate) const DEFAULT_POP: &str = "SJC";

pub struct Origin {
    /// This should match the name of a storage backend. See the the `Hosts` 
    /// section of the Fastly WASM service UI for more information.
    pub backend_name: &'static str,
    /// The name of the bucket to serve content from.
    pub bucket_name: &'static str,
    /// The host that the bucket is served on. This is used to make requests to the backend.
    pub bucket_host: &'static str,
}

/// Details of the origins. You must edit the bucket_names and bucket_hosts. Do not change
/// the backend_name.
pub(crate) const EU_ORIGIN: Origin = Origin {
    backend_name: "eu_origin",
    bucket_name: "YOUR-EU-BUCKET",
    bucket_host: "YOUR-EU-ENDPOINT",
};

pub(crate) const US_ORIGIN: Origin = Origin {
    backend_name: "us_origin",
    bucket_name: "YOUR-US-BUCKET",
    bucket_host: "YOUR-US-ENDPOINT",
};

lazy_static! {
    /// Regex for extracting region from endpoint
    pub(crate) static ref REGION_REGEX: Regex = Regex::new(r"^s3\.([[:alnum:]\-]+)\.backblazeb2\.com$").unwrap();
}

// If auth is required, configure your access keys in an edge dictionary named "bucket_auth":
// <backend_name>_access_key_id
// <backend_name>_secret_access_key

lazy_static! {
    /// Simple mapping from POP to origin:
    /// North America, South America, Asia/Pacific => US
    /// Europe, Africa => EU
    pub(crate) static ref POP_ORIGIN: HashMap<&'static str, Origin> = HashMap::from([
        ("AMS", EU_ORIGIN),
        ("WDC", US_ORIGIN),
        ("IAD", US_ORIGIN),
        ("BWI", US_ORIGIN),
        ("DCA", US_ORIGIN),
        ("ATL", US_ORIGIN),
        ("FTY", US_ORIGIN),
        ("PDK", US_ORIGIN),
        ("AKL", US_ORIGIN),
        ("BOG", US_ORIGIN),
        ("BOS", US_ORIGIN),
        ("BNE", US_ORIGIN),
        ("EZE", US_ORIGIN),
        ("CPT", EU_ORIGIN),
        ("MAA", US_ORIGIN),
        ("ORD", US_ORIGIN),
        ("LOT", US_ORIGIN),
        ("CHI", US_ORIGIN),
        ("MDW", US_ORIGIN),
        ("PWK", US_ORIGIN),
        ("CMH", US_ORIGIN),
        ("LCK", US_ORIGIN),
        ("CPH", EU_ORIGIN),
        ("CWB", US_ORIGIN),
        ("DFW", US_ORIGIN),
        ("DAL", US_ORIGIN),
        ("DEL", US_ORIGIN),
        ("DEN", US_ORIGIN),
        ("DTW", US_ORIGIN),
        ("DXB", US_ORIGIN),
        ("DUB", EU_ORIGIN),
        ("FOR", US_ORIGIN),
        ("FRA", EU_ORIGIN),
        ("HHN", EU_ORIGIN),
        ("FJR", US_ORIGIN),
        ("GNV", US_ORIGIN),
        ("ACC", EU_ORIGIN),
        ("HEL", EU_ORIGIN),
        ("HKG", US_ORIGIN),
        ("HNL", US_ORIGIN),
        ("IAH", US_ORIGIN),
        ("HYD", US_ORIGIN),
        ("JAX", US_ORIGIN),
        ("JNB", EU_ORIGIN),
        ("MCI", US_ORIGIN),
        ("CCU", US_ORIGIN),
        ("KUL", US_ORIGIN),
        ("LIM", US_ORIGIN),
        ("LCY", EU_ORIGIN),
        ("LHR", EU_ORIGIN),
        ("LON", EU_ORIGIN),
        ("LGB", US_ORIGIN),
        ("SMO", US_ORIGIN),
        ("BUR", US_ORIGIN),
        ("MAD", EU_ORIGIN),
        ("MAN", EU_ORIGIN),
        ("MNL", US_ORIGIN),
        ("MRS", EU_ORIGIN),
        ("MEL", US_ORIGIN),
        ("MIA", US_ORIGIN),
        ("MXP", EU_ORIGIN),
        ("LIN", EU_ORIGIN),
        ("MSP", US_ORIGIN),
        ("STP", US_ORIGIN),
        ("YUL", US_ORIGIN),
        ("BOM", US_ORIGIN),
        ("MUC", EU_ORIGIN),
        ("LGA", US_ORIGIN),
        ("EWR", US_ORIGIN),
        ("ITM", US_ORIGIN),
        ("OSL", EU_ORIGIN),
        ("PAO", US_ORIGIN),
        ("CDG", EU_ORIGIN),
        ("PER", US_ORIGIN),
        ("PHX", US_ORIGIN),
        ("PDX", US_ORIGIN),
        ("GIG", US_ORIGIN),
        ("FCO", EU_ORIGIN),
        ("SJC", US_ORIGIN),
        ("SCL", US_ORIGIN),
        ("CGH", US_ORIGIN),
        ("GRU", US_ORIGIN),
        ("SEA", US_ORIGIN),
        ("BFI", US_ORIGIN),
        ("ICN", US_ORIGIN),
        ("QPG", US_ORIGIN),
        ("SOF", EU_ORIGIN),
        ("STL", US_ORIGIN),
        ("BMA", EU_ORIGIN),
        ("SYD", US_ORIGIN),
        ("TYO", US_ORIGIN),
        ("HND", US_ORIGIN),
        ("NRT", US_ORIGIN),
        ("YYZ", US_ORIGIN),
        ("YVR", US_ORIGIN),
        ("VIE", EU_ORIGIN),
        ("WLG", US_ORIGIN),
    ]);
}
