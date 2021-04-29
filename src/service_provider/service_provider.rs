use ic_cdk::export::{candid::{CandidType, Deserialize}, Principal};
use ic_cdk::storage;
use ic_cdk_macros::*;
use std::collections::BTreeMap;

type IdStore = BTreeMap<String, Principal>;
type ServiceProviderStore = BTreeMap<Principal, ServiceProvider>;

#[ic_cdk_macros::query]
fn print() {
    ic_cdk::print("Hello World from DFINITY!");
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct ServiceProvider {
    pub name: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub contact_no: String,
    pub email: String
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct ServiceProviderDetail {
    pub list_of_items: Vec<String>,
}

#[query(name = "getSelf")]
fn get_self() -> ServiceProvider {
    let id = ic_cdk::caller();
    let service_provider_store = storage::get::<ServiceProviderStore>();

    service_provider_store
        .get(&id)
        .cloned()
        .unwrap_or_else(|| ServiceProvider::default())
}

#[query]
fn get(name: String) -> ServiceProvider {
    let id_store = storage::get::<IdStore>();
    let service_provider_store = storage::get::<ServiceProviderStore>();

    id_store
        .get(&name)
        .and_then(|id| service_provider_store.get(id).cloned())
        .unwrap_or_else(|| ServiceProvider::default())
}

#[update]
fn update(service_provider: ServiceProvider) {
    let principal_id = ic_cdk::caller();
    let id_store = storage::get_mut::<IdStore>();
    let service_provider_store = storage::get_mut::<ServiceProviderStore>();

    id_store.insert(service_provider.name.clone(), principal_id.clone());
    service_provider_store.insert(principal_id, service_provider);
}


#[query]
fn search(text: String) -> Option<&'static ServiceProvider> {
    let text = text.to_lowercase();
    let service_provider_store = storage::get::<ServiceProviderStore>();

    for (_, p) in service_provider_store.iter() {
        if p.name.to_lowercase().contains(&text) || p.description.to_lowercase().contains(&text) {
            return Some(p);
        }

        for x in p.keywords.iter() {
            if x.to_lowercase() == text {
                return Some(p);
            }
        }
    }

    None
}