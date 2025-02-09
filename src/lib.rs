use ecies::utils::generate_keypair;
use log::info;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use surrealdb::{engine::any::Any, Surreal};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[derive(Serialize, Deserialize, Clone)]
pub struct Keys {
    pub sk: String,
    pub pk: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Event {
    pub id: String,
    pub pk: String,
    pub content: String,
    pub ts: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub msg: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct File {
    pub name: String,
    pub bytes: Vec<u8>,
}

thread_local! {
    static NOSTR_CLIENT: RefCell<Option<nostr_sdk::Client>> = const { RefCell::new(None) };
    static SURREAL_DB: RefCell<Option<Surreal<Any>>> = const { RefCell::new(None) };
}

fn init_logging() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Info).unwrap();
}

async fn init_nostr_client(private_key: &str) {
    let keys = nostr_sdk::Keys::parse(private_key).unwrap();
    let client = nostr_sdk::Client::builder().signer(keys.clone()).build();
    client.add_relay("wss://relay.damus.io").await.unwrap();

    client.connect().await;
    let meta = nostr_sdk::Metadata::new()
        .name("wasmTestUser")
        .display_name("wasmTestUser");
    client.set_metadata(&meta).await.unwrap();

    NOSTR_CLIENT.with(|cl| {
        let mut client_ref = cl.borrow_mut();
        if client_ref.is_none() {
            *client_ref = Some(client);
        }
    });
}

fn get_db() -> Surreal<Any> {
    SURREAL_DB
        .with(|db| db.borrow().clone())
        .expect("is initialized")
}

fn get_nostr_client() -> nostr_sdk::Client {
    NOSTR_CLIENT
        .with(|client| client.borrow().clone())
        .expect("is initialized")
}

async fn init_surreal_db() {
    let db = surrealdb::engine::any::connect("indxdb://default")
        .await
        .unwrap();
    db.use_ns("").use_db("default").await.unwrap();
    SURREAL_DB.with(|surreal_db| {
        let mut db_ref = surreal_db.borrow_mut();
        if db_ref.is_none() {
            *db_ref = Some(db);
        }
    });
}

pub fn generate_nostr_keys() -> Keys {
    let keys = nostr_sdk::Keys::generate();
    Keys {
        sk: keys.secret_key().to_secret_hex(),
        pk: keys.public_key().to_hex(),
    }
}

pub fn generate_encryption_keys() -> Keys {
    let (sk, pk) = generate_keypair();
    Keys {
        sk: hex::encode(sk.serialize()),
        pk: hex::encode(pk.serialize()),
    }
}

#[wasm_bindgen]
pub async fn initialize() -> String {
    init_logging();
    init_surreal_db().await;
    if let Some(nostr_keys) = get_nostr_keys_from_db().await {
        init_nostr_client(&nostr_keys.sk).await;
        nostr_keys.pk
    } else {
        let nostr_keys = generate_nostr_keys();
        save_nostr_keys_to_db(&nostr_keys).await;
        save_encryption_keys_to_db(&generate_encryption_keys()).await;
        nostr_keys.pk
    }
}

#[wasm_bindgen]
pub async fn send_nostr_msg(msg: &str) {
    let msg = msg.to_owned();
    let msg_clone = msg.clone();
    spawn_local(async move {
        let event_builder = nostr_sdk::EventBuilder::text_note(msg_clone);
        let event_id = get_nostr_client()
            .send_event_builder(event_builder)
            .await
            .unwrap();
        info!("sent event, event id: {}", event_id.id());
    });
    let encryption_keys = get_encryption_keys_from_db().await.unwrap();
    let encrypted = hex::encode(encrypt(msg.as_bytes(), &encryption_keys.pk));
    save_encrypted_msg(&encrypted).await;
}

#[wasm_bindgen]
pub async fn fetch_and_decrypt_local_messages() -> JsValue {
    let encryption_keys = get_encryption_keys_from_db().await.unwrap();
    let msgs = fetch_messages().await;
    let decrypted: Vec<String> = msgs
        .into_iter()
        .map(|msg| {
            std::str::from_utf8(&decrypt(
                &hex::decode(msg.msg.as_bytes()).unwrap(),
                &encryption_keys.sk,
            ))
            .unwrap()
            .to_owned()
        })
        .collect();
    serde_wasm_bindgen::to_value(&decrypted).unwrap()
}

async fn fetch_messages() -> Vec<Message> {
    let db = get_db();
    let msgs: Vec<Message> = db.select("msg").await.unwrap();
    msgs
}

#[wasm_bindgen]
pub async fn save_image(file_name: &str, file_bytes: Vec<u8>) {
    let encryption_keys = get_encryption_keys_from_db().await.unwrap();
    let db = get_db();
    let f = File {
        name: file_name.to_owned(),
        bytes: encrypt(&file_bytes, &encryption_keys.pk),
    };
    let _: Option<File> = db.create("img").content(f).await.unwrap();
}

#[wasm_bindgen]
pub async fn fetch_images() -> JsValue {
    let encryption_keys = get_encryption_keys_from_db().await.unwrap();
    let db = get_db();
    let files: Vec<File> = db.select("img").await.unwrap();
    let decrypted: Vec<File> = files
        .into_iter()
        .map(|f| File {
            name: f.name,
            bytes: decrypt(&f.bytes, &encryption_keys.sk),
        })
        .collect();
    serde_wasm_bindgen::to_value(&decrypted).unwrap()
}

async fn save_encrypted_msg(encrypted: &str) {
    let db = get_db();
    let _: Option<Message> = db
        .create("msg")
        .content(Message {
            msg: encrypted.to_owned(),
        })
        .await
        .unwrap();
}

async fn get_nostr_keys_from_db() -> Option<Keys> {
    let db = get_db();
    let res: Option<Keys> = db.select(("keys", "nostr")).await.unwrap();
    res
}

async fn save_nostr_keys_to_db(keys: &Keys) {
    let db = get_db();
    let _: Option<Keys> = db
        .create(("keys", "nostr"))
        .content(keys.clone())
        .await
        .unwrap();
}

async fn get_encryption_keys_from_db() -> Option<Keys> {
    let db = get_db();
    let res: Option<Keys> = db.select(("keys", "encryption")).await.unwrap();
    res
}

async fn save_encryption_keys_to_db(keys: &Keys) {
    let db = get_db();
    let _: Option<Keys> = db
        .create(("keys", "encryption"))
        .content(keys.clone())
        .await
        .unwrap();
}

fn encrypt(input: &[u8], key: &str) -> Vec<u8> {
    let decoded_key = hex::decode(key).unwrap();
    ecies::encrypt(&decoded_key, input).unwrap()
}

fn decrypt(input: &[u8], key: &str) -> Vec<u8> {
    let decoded_key = hex::decode(key).unwrap();
    let decoded_msg = input;
    ecies::decrypt(&decoded_key, &decoded_msg).unwrap()
}

#[wasm_bindgen]
pub async fn fetch_nostr_events(from: &str) -> Result<JsValue, JsValue> {
    let filter = nostr_sdk::Filter::new()
        .author(nostr_sdk::PublicKey::parse(from).unwrap())
        .kind(nostr_sdk::Kind::TextNote);
    let events = get_nostr_client()
        .fetch_events(filter, std::time::Duration::from_secs(10))
        .await
        .unwrap();
    Ok(serde_wasm_bindgen::to_value(
        &events
            .into_iter()
            .map(|e| Event {
                id: e.id.to_hex(),
                pk: e.pubkey.to_hex(),
                content: e.content,
                ts: e.created_at.as_u64(),
            })
            .collect::<Vec<Event>>(),
    )
    .unwrap())
}
