use ic_cdk::api::call::CallResult;
use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use ic_cdk::storage;
use ic_cdk_macros::*;
use serde_bytes::ByteBuf;
use std::collections::BTreeMap;

mod address;
mod metadata_attribute;
mod rand;

use crate::address::AddressBook;
use crate::metadata_attribute::{Attribute, AttributeData};

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct TransferNotification {
    pub to: Principal,
    pub token_id: u64,
    pub from: Principal,
    pub amount: u64,
}

#[init]
fn init() {
    ic_cdk::println!("Init {:?}", ic_cdk::api::time());

    let address_book = storage::get_mut::<AddressBook>();
    address_book.total_supply = 8000;

    //needs to be a way to pass this into init
    let owner_id =
        Principal::from_text("2c22g-lboam-nseoa-i5al6-o7k6f-o2fwz-huoua-be63r-oi3k2-wy7uq-zae")
            .unwrap();
    address_book.add_controller(&owner_id);

    init_attribute();
}

fn init_attribute() -> () {
    let attribute = storage::get_mut::<Attribute>();

    attribute.weapons = vec![
        "weapons1",
        "weapons2",
        "weapons3",
        "weapons4",
        "weapons5",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    attribute.chest = vec![
        "chest1",
        "chest2",
        "chest3",
        "chest4",
        "chest5",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    attribute.head = vec![
        "head1",
        "head2",
        "head3",
        "head4",
        "head5",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    attribute.waist = vec![
        "waist1",
        "waist2",
        "waist3",
        "waist4",
        "waist5",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    attribute.foot = vec![
        "foot1",
        "foot2",
        "foot3",
        "foot4",
        "foot5",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    attribute.underwear = vec![
        "underwear1",
        "underwear2",
        "underwear3",
        "underwear4",
        "underwear5",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    attribute.accessory = vec![
        "accessory1",
        "accessory2",
        "accessory3",
        "accessory4",
        "accessory5",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    attribute.pants = vec![
        "pants1",
        "pants2",
        "pants3",
        "pants4",
        "pants5",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    attribute.prefixes = vec![
        "prefixes1",
        "prefixes2",
        "prefixes3",
        "prefixes4",
        "prefixes5",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    attribute.name_prefixes = vec![
        "name_prefixes1",
        "name_prefixes2",
        "name_prefixes3",
        "name_prefixes4",
        "name_prefixes5",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    attribute.name_suffixes = vec![
        "name_suffixes1",
        "name_suffixes2",
        "name_suffixes3",
        "name_suffixes4",
        "name_suffixes5",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

#[query]
fn user_tokens(user: Principal) -> Vec<u64> {
    return storage::get::<AddressBook>().user_tokens(&user);
}

#[query]
fn supply() -> u64 {
    return storage::get::<AddressBook>().total_supply;
}

#[query]
fn remaining() -> u64 {
    return storage::get::<AddressBook>().remaining();
}

#[query]
fn owner_of(token_id: u64) -> Option<Principal> {
    return storage::get::<AddressBook>().owner_of(&token_id);
}

#[update]
fn transfer_to(user: Principal, token_id: u64) -> bool {
    return storage::get_mut::<AddressBook>().transfer_to(user, token_id);
}

#[update]
async fn transfer_with_notify(user_id: Principal, token_id: u64) -> bool {
    let address_book = storage::get_mut::<AddressBook>();
    if address_book.transfer_to(user_id, token_id) {
        match ic_cdk::call(
            user_id,
            "transfer_notification",
            (TransferNotification {
                to: user_id,
                from: ic_cdk::caller(),
                token_id: token_id,
                amount: 1,
            },),
        )
        .await as CallResult<()>
        {
            Ok(_) => return true,
            Err(_) => {
                //gets in rejected state and the next
                //line is not executed completely
                //address_book.undo_transfer(user_id, token_id);
                return false;
            }
        }
    } else {
        return false;
    }
}

#[update]
fn claim() -> Result<u64, String> {
    //return Err("No claims for this NFT type (IC DRIP)".to_string());
    return storage::get_mut::<AddressBook>().claim(ic_cdk::caller());
}

//Allow the original airdrop to always exists for future references
//where sites can use this to know if the person transferred their NFT or not.
#[query]
fn get_airdrops() -> Vec<(u64, bool)> {
    let airdroppers = storage::get_mut::<BTreeMap<Principal, Vec<u64>>>();
    let address_book = storage::get_mut::<AddressBook>();
    match airdroppers.get(&ic_cdk::caller()) {
        Some(tokens) => {
            let mut results: Vec<(u64, bool)> = Vec::new();
            for token in tokens {
                results.push((
                    token.clone(),
                    address_book.is_owner_of(ic_cdk::caller(), token),
                ));
            }
            return results;
        }
        None => Vec::new(),
    }
}

//Save list of airdrops for other platforms to use.
fn update_airdroppers(user: Principal, token_id: u64) -> () {
    let airdroppers = storage::get_mut::<BTreeMap<Principal, Vec<u64>>>();
    match airdroppers.get_mut(&user) {
        Some(tokens) => tokens.push(token_id),
        None => {
            airdroppers.insert(user, vec![token_id]);
        }
    }
}

#[update(guard = "is_controller")]
fn add_airdrops(users: Vec<Principal>) -> bool {
    let address_book = storage::get_mut::<AddressBook>();
    for id in users {
        match address_book.claim(id) {
            Ok(token_id) => update_airdroppers(id, token_id),
            Err(_) => return false,
        }
    }
    return true;
}

#[update(guard = "is_controller")]
fn add_controller(user: Principal) -> bool {
    return storage::get_mut::<AddressBook>().add_controller(&user);
}

#[update(guard = "is_controller")]
fn remove_controller(user: Principal) -> bool {
    return storage::get_mut::<AddressBook>().remove_controller(&user);
}

#[update(guard = "is_controller")]
fn get_controllers() -> Vec<Principal> {
    return storage::get::<AddressBook>().controllers.clone();
}

#[update]
fn get_cycles() -> u64 {
    return ic_cdk::api::canister_balance();
}

#[query]
fn name() -> String {
    return "ICD_NFT".to_string();
}

#[query]
fn symbol() -> String {
    return "ICDN".to_string();
}

#[query]
fn get_address_book() -> AddressBook {
    return storage::get::<AddressBook>().clone();
}

type HeaderField = (String, String);

#[derive(Clone, Debug, CandidType, Deserialize)]
struct HttpRequest {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: ByteBuf,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct HttpResponse {
    status_code: u16,
    headers: Vec<HeaderField>,
    body: Vec<u8>,
}

#[query]
async fn http_request(req: HttpRequest) -> HttpResponse {
    let parts: Vec<&str> = req.url.split('?').collect();

    let token_param: Vec<&str> = parts[1].split('=').collect();
    let token_id = token_param[1].parse::<u64>().unwrap();

    let address_book = storage::get_mut::<AddressBook>();

    if token_id <= 0 || token_id > address_book.total_supply || !address_book.is_claimed(&token_id)
    {
        return HttpResponse {
            status_code: 404,
            headers: Vec::new(),
            body: Vec::new(),
        };
    }

    let attribute = storage::get_mut::<Attribute>();

    let seed = address_book.token_seeds.get(&token_id).unwrap();

    let data = attribute.generate(token_id.clone() + seed.clone());

    let results = data.as_bytes();

    let mut headers: Vec<HeaderField> = Vec::new();
    headers.push(("content-type".to_string(), "image/svg+xml".to_string()));
    headers.push((
        "cache-control".to_string(),
        "public, max-age=604800, immutable".to_string(),
    ));
    return HttpResponse {
        status_code: 200,
        headers,
        body: results.to_vec(),
    };
}

#[query]
fn data_of(token_id: u64) -> Vec<AttributeData> {
    let address_book = storage::get::<AddressBook>();
    if token_id <= 0 || token_id > address_book.total_supply || !address_book.is_claimed(&token_id)
    {
        return Vec::new();
    }
    let seed = address_book.token_seeds.get(&token_id).unwrap();
    let attribute = storage::get::<Attribute>();
    return attribute.get_attribute_datas(token_id + seed);
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum DataOfQuery {
    Range(u64, u64),
    List(Vec<u64>),
}

#[query]
fn data_of_many(query: DataOfQuery) -> BTreeMap<u64, Vec<AttributeData>> {
    let address_book = storage::get::<AddressBook>();
    match query {
        DataOfQuery::Range(from, to) => {
            let mut results = BTreeMap::new();
            for i in from..to + 1 {
                if !address_book.is_claimed(&i) {
                    continue;
                }
                let seed = address_book.token_seeds.get(&i).unwrap();
                let attribute = storage::get::<Attribute>();
                results.insert(i, attribute.get_attribute_datas(i + seed));
            }
            return results;
        }
        DataOfQuery::List(items) => {
            let mut results = BTreeMap::new();
            for id in items {
                if !address_book.is_claimed(&id) {
                    continue;
                }
                let seed = address_book.token_seeds.get(&id).unwrap();
                let attribute = storage::get::<Attribute>();
                results.insert(id, attribute.get_attribute_datas(id + seed));
            }
            return results;
        }
    }
}

#[query]
fn get_token_properties(token_id: u64) -> Vec<(String, String)> {
    let address_book = storage::get::<AddressBook>();
    if token_id <= 0 || token_id > address_book.total_supply || !address_book.is_claimed(&token_id)
    {
        return Vec::new();
    }
    let seed = address_book.token_seeds.get(&token_id).unwrap();
    let attribute = storage::get::<Attribute>();
    return attribute.get_properties(token_id + seed);
}

#[query]
fn get_token_properties_range(from: u64, to: u64) -> Vec<Vec<(String, String)>> {
    let address_book = storage::get::<AddressBook>();
    let mut results = Vec::new();
    for i in from..to + 1 {
        if !address_book.is_claimed(&i) {
            continue;
        }
        let seed = address_book.token_seeds.get(&i).unwrap();
        let attribute = storage::get::<Attribute>();
        results.push(attribute.get_properties(i + seed))
    }
    return results;
}

//this is not working correctly.
#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    return r#"
            type HeaderField = record { text; text; };
            type HttpRequest = record {
                method: text;
                url: text;
                headers: vec HeaderField;
                body: blob;
            };
            type HttpResponse = record {
                status_code: nat16;
                headers: vec HeaderField;
                body: blob;
            };
            type AddressBook = record {
                total_supply: nat64;
                tokens: vec record { nat64; principal};
                controllers: vec principal;
                claim_index: nat64;
                token_seeds: vec record { nat64; nat64};
            };
            type ClaimResult = variant {
                Ok : nat64;
                Err: text;
            };
            type TransferNotification = record {
                to: principal;
                from: principal;
                token_id: nat64;
                amount: nat64;
            };
            type AttributeData = record {
                slot: text;
                name: text;
                prefix: text;
                name_prefix: text;
                name_suffix: text;
                special: bool;
            };
            type DataOfQuery = variant {
                Range: record {nat64; nat64};
                List: vec nat64;
            };
            service : {
                http_request: (request: HttpRequest) -> (HttpResponse) query;
                get_address_book: () -> (AddressBook) query;
                get_token_properties: (nat64) -> (vec record { text; text}) query;
                get_token_properties_range: (nat64, nat64) -> (vec vec record { text; text}) query;
                data_of: (nat64) -> (vec AttributeData) query;
                data_of_many: (DataOfQuery) -> (vec record {nat64; vec AttributeData;}) query;
                get_cycles: () -> (nat64);
                get_airdrops: () -> (vec record { nat64; bool }) query;
                add_airdrops: (vec principal) -> (bool);
                name: () -> (text) query;
                symbol: () -> (text) query;
                user_tokens: (principal) -> (vec nat64) query;
                owner_of: (nat64) -> (opt principal) query;
                transfer_to: (principal, nat64) -> (bool);
                transfer_with_notify: (principal, nat64) -> (bool);
                claim: () -> (ClaimResult);
                remaining: () -> (nat64);
                get_controllers: () -> (vec principal) query;
                add_controller: (principal) -> (bool);
                remove_controller: (principal) -> (bool);
                supply: () -> (nat64);
            }
    "#
    .to_string();
}

#[derive(CandidType, Deserialize)]
struct StableStorage {
    address_book: AddressBook,
    airdroppers: BTreeMap<Principal, Vec<u64>>,
}

#[pre_upgrade]
fn pre_upgrade() {
    let stable = StableStorage {
        address_book: storage::get::<AddressBook>().clone(),
        airdroppers: storage::get::<BTreeMap<Principal, Vec<u64>>>().clone(),
    };

    match storage::stable_save((stable,)) {
        Ok(_) => (),
        Err(candid_err) => {
            ic_cdk::trap(&format!(
                "An error occurred when saving to stable memory (pre_upgrade): {}",
                candid_err
            ));
        }
    };
}

#[post_upgrade]
fn post_upgrade() {
    init();
    if let Ok((storage,)) = storage::stable_restore::<(StableStorage,)>() {
        let address_book = storage::get_mut::<AddressBook>();
        *address_book = storage.address_book;
        let airdroppers = storage::get_mut::<BTreeMap<Principal, Vec<u64>>>();
        *airdroppers = storage.airdroppers;
    }
}

fn is_controller() -> Result<(), String> {
    if storage::get::<AddressBook>().is_controller(&ic_cdk::caller()) {
        Ok(())
    } else {
        Err("Only the controller can call this method.".to_string())
    }
}