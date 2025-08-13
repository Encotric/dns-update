/*
 * Copyright Stalwart Labs LLC See the COPYING
 * file at the top-level directory of this distribution.
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use std::time::Duration;

use hickory_client::rr::resource::RecordRef;
use serde::{Deserialize, Serialize};

use crate::{http::HttpClientBuilder, strip_origin_from_name, DnsRecord, DnsRecordType, IntoFqdn};

#[derive(Clone)]
pub struct BunnyProvider {
    client: HttpClientBuilder,
    endpoint: String,
}

/// The parameters for create requests to the bunny API.
#[derive(Serialize, Clone, Debug)]
pub struct CreateDnsRecordParams {
    pub zone_id: i64,
    #[serde(rename = "type")]
    pub rr_type: RecordType,
    pub ttl: Option<u32>,
    pub name: String,
    pub data: RecordData,
}

/// The parameters for delete requests to the bunny API.
#[derive(Serialize, Clone, Debug)]
pub struct DeleteDnsRecordParams {
    pub zone_id: i64,
    #[serde(rename = "id")]
    pub record_id: i64,
}

#[repr(u8)]
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub enum RecordType {
    A = 0,
    AAAA = 1,
    CNAME = 2,
    TXT = 3,
    MX = 4,
    SRV = 8,
}

#[derive(Serialize, Clone, Debug)]
pub enum RecordData {
    A { value: String },
    AAAA { value: String },
    CNAME { value: String },
    TXT { value: String },
    MX { priority: u16, value: String },
    SRV { priority: u16, weight: u16, port: u16, value: String },
}

/// The response for creation and modification requests of the bunny API.
#[derive(Deserialize, Debug)]
pub struct BunnyCreateApiResponse {
    pub created: String,
    pub domain: String,
    pub subname: String,
    pub name: String,
    pub records: Vec<String>,
    pub ttl: u32,
    #[serde(rename = "type")]
    pub record_type: String,
    pub touched: String,
}

#[derive(Deserialize)]
struct BunnyEmptyResponse {}

/// The default endpoint for the bunny API.
const DEFAULT_API_ENDPOINT: &str = " https://api.bunny.net/";
