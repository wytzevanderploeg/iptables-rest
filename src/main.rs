use iptables::IPTables;
use pnet::datalink::interfaces;
use rocket::{delete, get, launch, post, put, routes};
use rocket::serde::json::Json;

use crate::command::InsertRuleCommand;
use crate::dto::{DataMap, InterfaceDto, TablesResponse};

mod dto;
mod command;

/// Get a list of interfaces.
#[get("/interface")]
pub fn get_interfaces() -> Json<DataMap<Vec<InterfaceDto>>> {
    let all_interfaces = interfaces();
    let result: Vec<InterfaceDto> = all_interfaces.iter()
        .map(|intf| InterfaceDto::from_network_interface(intf))
        .collect();
    return Json(DataMap { data: result });
}

/// Get the default network interface.
/// @see: https://docs.rs/pnet/latest/pnet/datalink/fn.interfaces.html
#[get("/interface/default")]
pub fn get_default_interface() -> Json<InterfaceDto> {
    let all_interfaces = interfaces();
    let default_interface = all_interfaces.iter()
        .find(|intf| intf.is_up() && !intf.is_loopback() && !intf.ips.is_empty())
        .unwrap();
    return Json(InterfaceDto::from_network_interface(default_interface));
}

/// Return a list of the default tables.
#[get("/")]
pub fn get_tables() -> Json<TablesResponse> {
    let tables: Vec<String> = vec![
        "filter".to_string(),
        "mangle".to_string(),
        "nat".to_string(),
        "raw".to_string(),
        "security".to_string()
    ];
    return Json(TablesResponse {tables});
}

/// Returns all chains for the given table
#[get("/<table>")]
pub fn get_chains(table: &str) -> Json<DataMap<Vec<String>>> {
    let ipt: IPTables = get_iptables();
    let chains: Vec<String> = ipt.list_chains(table).unwrap();
    return Json(DataMap { data: chains });
}

/// Adds a new chain to the table
#[put("/<table>", format="json", data = "<chain>")]
fn add_chain(table: &str, chain: Json<DataMap<String>>) -> Json<DataMap<String>> {
    let ipt: IPTables = get_iptables();
    let chain_name = &chain.0.data.to_uppercase();
    match ipt.new_chain(table, chain_name) {
        Ok(_) => Json(DataMap {data: chain_name.clone()}),
        Err(e) => panic!("Could not add chain: {:?}", e),
    }
}

/// Returns all rules for the given chain
#[get("/<table>/<chain>")]
fn get_chain(table: &str, chain: &str) -> Json<DataMap<Vec<String>>> {
    let rules = get_chain_ignore_case(&table, &chain);
    return Json(DataMap { data: rules });
}

/// Deletes the given chain.
#[delete("/<table>/<chain>")]
fn delete_chain(table: &str, chain: &str) -> Json<DataMap<bool>> {
    let ipt: IPTables = get_iptables();
    let chain_name = &chain.to_uppercase();
    match ipt.delete_chain(&table, &chain_name) {
        Ok(_) => return Json(DataMap { data: true }),
        Err(e) => panic!("Could not delete chain: {:?}", e)
    }
}

/// Sets the policy for the given chain.
#[post("/<table>/<chain>/policy", format = "json", data = "<rule>")]
fn set_policy(table: &str, chain: &str, rule: Json<DataMap<String>>) -> Json<DataMap<String>> {
    let ipt: IPTables = get_iptables();
    let policy = &rule.data;
    match ipt.set_policy(table, chain, policy) {
        Ok(_) => Json(DataMap {data: "ok".to_string()}),
        Err(e) => panic!("Could not append rule: {:?}", e)
    }
}

/// Appends a rule to the given chain.
#[put("/<table>/<chain>", format = "json", data = "<rule>")]
fn append_rule(table: &str, chain: &str, rule: Json<DataMap<String>>) -> Json<DataMap<String>> {
    let ipt: IPTables = get_iptables();
    let rule = &rule.data;
    match ipt.append(table, chain.to_uppercase().as_str(), rule) {
        Ok(_) => Json(DataMap {data: "ok".to_string()}),
        Err(e) => panic!("Could not append rule: {:?}", e)
    }
}

/// Inserts a rule into the given chain
#[post("/<table>/<chain>", format = "json", data = "<command>")]
fn insert_rule(table: &str, chain: &str, command: Json<InsertRuleCommand>) -> Json<DataMap<String>> {
    let ipt: IPTables = get_iptables();
    let rule = &command.rule;
    let position = command.position as i32;
    match ipt.insert(table, chain, rule, position) {
        Ok(_) => Json(DataMap {data: "ok".to_string()}),
        Err(e) => panic!("Could not insert rule: {:?}", e)
    }
}

#[delete("/<table>/<chain>/delete/<position>", format = "json")]
fn delete_rule(table: &str, chain: &str, position: u32) -> Json<DataMap<String>> {
    let ipt: IPTables = get_iptables();
    let rules = get_chain_ignore_case(&table, chain);
    let rule = rules.get(usize::try_from(position).unwrap());
    match rule {
        Some(val) => {
            let _ = ipt.delete(&table, &chain, val);
            Json(DataMap {data: "ok".to_string()})
        },
        None => panic!("No rule at position: {}", position)
    }
}

/// Returns a handle to the IPTables object.
fn get_iptables() -> IPTables {
    return iptables::new(false)
        .unwrap();
}

/// Lists the rules for the given chain (chain name is case insensitive).
fn get_chain_ignore_case(table: &str, chain: &str) -> Vec<String> {
    let ipt = get_iptables();
    let chains = ipt.list_chains(&table).unwrap();
    let result = chains.iter()
        .find(|c| c.eq_ignore_ascii_case(chain))
        .map(|c| ipt.list(&table, c))
        .unwrap();
    return match result {
        Ok(data) => data,
        Err(e) => panic!("Could not list rules in chain: {:?}", e)
    };
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            get_interfaces,
            get_default_interface,

            get_tables,
            get_chains,

            get_chain,
            add_chain,
            delete_chain,

            set_policy,
            append_rule,
            insert_rule,
            delete_rule
        ])
}