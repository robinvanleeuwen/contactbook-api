use jsonrpc_http_server::ServerBuilder;
use std::net::SocketAddr;
use serde_json::*;
use serde::{Serialize, Deserialize};
use jsonrpc_core::*;

extern crate contactbook_api;
extern crate diesel;

use contactbook_api::*;
use self::models::*;
use diesel::prelude::*;
use diesel::insert_into;
use diesel::delete;
use jsonrpc_http_server::DomainsValidation;
use jsonrpc_http_server::AccessControlAllowOrigin;

fn main() {

    let mut io: IoHandler = IoHandler::new();

    io = add_handlers(io);

    let address = get_listen_address();
    let port: u16 = get_listen_port();

    let socket_address = &SocketAddr::new(
        (address).parse().unwrap(), port
    );

    let server = ServerBuilder::new(io)
        .threads(5)
        .cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Any]))
        .start_http(socket_address)
        .expect("Unable to start server.");

    println!("Starting server on {}:{}...", address, port);

    server.wait();
}

// Add API end-points to the JSONRPC IoHandler,
// returns the given IoHandler with added methods
fn add_handlers(mut io: IoHandler) -> IoHandler {

    io.add_method("create", create_contact);
    io.add_method("delete", delete_contact);
    io.add_method("search", show_contacts);
    io
}


fn delete_contact(params: Params) -> jsonrpc_core::Result<jsonrpc_core::types::Value>{
    use self::schema::contact::dsl::*;

    let connection = match establish_connection() {
        Ok(c) => c,
        Err(_) => {
            let error = "500 ERROR: Could not connect to database.".to_string();
            return Ok(Value::String(error));
        }
    };

    #[derive(Deserialize, Serialize, Debug)]
    struct DeleteContactParams {
        id: i32,
    }

    let p: DeleteContactParams = params.parse()?;

    let num_deleted = delete(contact.filter(id.eq(p.id)))
        .execute(&connection)
        .expect("Failure to delete");

    Ok(Value::String(num_deleted.to_string()))
}


fn show_contacts(params: Params) -> jsonrpc_core::Result<jsonrpc_core::types::Value>{

    use self::schema::contact::dsl::*;

    let connection = match establish_connection() {
        Ok(c) => c,
        Err(_) => {
            let error = "500 ERROR: Could not connect to database.".to_string();
            return Ok(Value::String(error));
        }
    };

    #[derive(Deserialize, Serialize, Debug)]
    struct ShowContactParams {
        name: String,
        comment: Option<String>,
    }

    let p: ShowContactParams = params.parse()?;

    let pattern = format!("%{}%", p.name);

    let result = contact
        .filter(name.ilike(pattern))
        .limit(5)
        .load::<Contact>(&connection)
        .expect("Error!");

    println!("Found {} contacts", result.len());

    for c in &result {
        println!("- {}", c.name);
    }

    let mut cts = Vec::new();
    let mut data = json!({});

    for c in &result { cts.push(c); }

    if !cts.is_empty() {
        data = json!(
            {
                "contacts": cts
            }
        );
    }

    Ok(data)
}

fn create_contact(params: Params) -> jsonrpc_core::Result<jsonrpc_core::types::Value>{

    use schema::contact;

    let connection = match establish_connection() {
        Ok(c) => c,
        Err(_) => {
            let error = "500 ERROR: Could not connect to database.".to_string();
            return Ok(Value::String(error));
        }
    };

    #[derive(Deserialize, Serialize, Debug)]
    struct CreateContactParams {
        name: String,
        phone1: Option<String>,
        phone2: Option<String>,
        phone3: Option<String>,
        email: Option<String>,
        comment: Option<String>,
    }

    let p: CreateContactParams = params.parse()?;

    let cname    = p.name;
    let cphone1  = p.phone1.unwrap_or_else(|| "".to_string());
    let cphone2  = p.phone2.unwrap_or_else(|| "".to_string());
    let cphone3  = p.phone3.unwrap_or_else(|| "".to_string());
    let cemail   = p.email.unwrap_or_else(|| "".to_string());
    let ccomment = p.comment.unwrap_or_else(|| "".to_string());

    let new_contact = CreateContact {
        name: cname,
        phone1: cphone1,
        phone2: cphone2,
        phone3: cphone3,
        email: cemail,
        comment: ccomment,
    };

    insert_into(contact::table)
        .values(&new_contact)
        .execute(&connection)
        .expect("Could not insert");

    Ok(Value::String("CREATED".into()))
}

