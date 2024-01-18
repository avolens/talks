#![allow(non_snake_case)]
use axum::{response::IntoResponse, routing::post, Json, Router};
use axum_server::tls_rustls::RustlsConfig;
use kube::{
    api::DynamicObject,
    core::admission::{AdmissionResponse, AdmissionReview},
    CustomResource,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fmt::Write, net::SocketAddr, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
struct PortEntry {
    name: Option<String>,
    containerPort: Option<u16>,
    hostPort: Option<u16>,
    hostIP: Option<String>,
    protocol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
struct ContainerType {
    name: String,
    image: String,
    ports: Vec<PortEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
struct SecContext {
    privileged: Option<bool>,
}

#[derive(CustomResource, Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[kube(
    group = "kube.rs",
    version = "v1",
    kind = "BaseAdmissionReview",
    namespaced
)]
pub struct TestSpec {
    containers: Vec<ContainerType>,
}

#[tokio::main]
pub async fn main() {
    pretty_env_logger::init();

    let config = RustlsConfig::from_pem_file(
        PathBuf::from("certs/server.crt"),
        PathBuf::from("certs/server.key"),
    )
    .await
    .unwrap();

    let port = 443;
    // Create the TLS configuration
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let app = Router::new().route("/validate", post(validate));

    println!("binding talk AC server to 0.0.0.0:{port}");

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn get_value(obj: &Value, path: &str) -> Option<Value> {
    let mut obj = obj.clone();
    for part in path.split(".") {
        println!("getting part {} from {}", part, obj);
        match obj {
            Value::Object(m) => {
                obj = m.get(part).unwrap().clone();
            }
            _ => {
                panic!("not an object")
            }
        }
    }

    return Some(obj);
}

async fn validate(Json(mut review): Json<AdmissionReview<DynamicObject>>) -> impl IntoResponse {
    println!("validating..");

    let request = review.request.as_ref().unwrap();
    let mut err = String::new();

    // in the object portion, there is the spec portion.
    let object = request.object.as_ref().unwrap().data.as_object().unwrap();

    let object = serde_json::Value::Object(object.clone());

    // check that every speaker can talk

    let mut all_ready = true;

    match get_value(&object, "spec.speaker") {
        Some(speakers) => {
            for speaker in speakers.as_array().unwrap() {
                println!("processing speaker: {}", speaker);

                match get_value(&speaker, "canTalk") {
                    Some(canTalk) => {
                        if canTalk.as_bool().unwrap() {
                            println!("speaker can talk");
                        } else {
                            println!("speaker cannot talk");
                            all_ready = false;
                        }
                    }
                    None => {
                        println!("no cantalk set");
                        all_ready = false;
                    }
                }
            }
        }
        None => {}
    }

    if !all_ready {
        write!(err, "not all speakers are ready").ok();
    }
    // is the talk accepted?

    if err.is_empty() {
        match get_value(&object, "spec.status.accepted") {
            Some(accepted) => {
                if accepted.as_bool().unwrap() {
                    println!("talk is accepted");
                } else {
                    write!(err, "talk is not accepted").ok();
                }
            }
            None => {
                write!(err, "accepted not present").ok();
            }
        }
    }

    // is the talk in the schedule?

    if err.is_empty() {
        match get_value(&object, "spec.status.timestamp") {
            Some(ts) => {
                let ts = ts.as_u64().unwrap();
                if ts >= 1688001031602002 {
                    write!(err, "talk has invalid schedule").ok();
                } else {
                    println!("talk is scheduled correctly");
                }
            }
            None => {
                write!(err, "talk is not scheduled").ok();
            }
        }
    }

    let mut response = AdmissionResponse::from(request);

    println!("err: {}", err);

    response.allowed = err.is_empty();
    response.result.message = err;

    review.response = Some(response);

    Json(review)
}
