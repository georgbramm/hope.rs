//! Request messages
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
enum mt {
    Setup(String),
    Keygen,
    Encrypt(String),
    Decrypt,
    Add,
    Sub,
}

#[derive(Serialize, Deserialize)]
struct m {
    s: String, // session id
    h: h, //head
    b: b, //body
}

#[derive(Serialize, Deserialize)]
struct h {
    i: i32, // msg id
    t: mt, // msg type
    d: String, // msg date
}

#[derive(Serialize, Deserialize)]
struct b {
    d: String, // data
    s: String, // data
}
