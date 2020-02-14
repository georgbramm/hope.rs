//! Everything related to the actual server implementation

use ::hope::config::Config;
use actix::{prelude::*, SystemRunner};
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    http::header::{CONTENT_TYPE, LOCATION},
    guard, middleware,web, Error,
    web::{get, post, resource},
    App, HttpResponse, HttpServer, HttpRequest
};
use failure::{format_err, Fallible};
use num_cpus;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use std::{
    net::{SocketAddr, ToSocketAddrs},
    slice::from_ref,
    thread,
};
use url::Url;
use hope_library::scheme::hope;
use hope_library::websocket::HopeWebSocket;
use std::time::{Duration, Instant};
use actix::prelude::*;
use actix_files as fs;
use actix_web_actors::ws;
use actix_web::http::{header, Method, StatusCode};
use serde::{Deserialize, Serialize};
//use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::io;

const HTML_FOLDER: &'static str = "static/html/";
const JS_FOLDER: &'static str = "static/js/";
const FILE_INDEX: &'static str = "index.html";
const FILE_NOTFOUND: &'static str = "404.html";

/// 404 handler
async fn p404() -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open([HTML_FOLDER, FILE_NOTFOUND].concat())?.set_status_code(StatusCode::NOT_FOUND))
}

/// do websocket handshake and start `MyWebSocket` actor
async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("{:?}", r);
    let wsh = HopeWebSocket::new();
    let _hope: hope = hope::new(String::from("local"), &wsh);
    let res = ws::start(wsh, &r, stream);
    //let res = HopeWebSocket::create(|ctx: &mut Context<HopeWebSocket>| HopeWebSocket { hb: Instant::now() });
    println!("{:?}", res);
    res
}

/// The server instance
pub struct Server {
    config: Config,
    runner: SystemRunner,
    url: Url,
}

impl Server {
    /// Create a new server instance
    pub fn from_config(config: &Config) -> Fallible<Self> {
    	std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
	    env_logger::init();
        // Build a new actor system
        let runner = actix::System::new("backend");

        // Create the server
        let server = HttpServer::new(move || {
            App::new()
                .wrap(
                    Cors::new()
                        .allowed_methods(vec!["GET", "POST"])
                        .allowed_header(CONTENT_TYPE)
                        .max_age(3600)
                        .finish(),
                )
	            .wrap(middleware::Logger::default())
	            //global
	            .data(web::JsonConfig::default().limit(1024 * 1024)) // <- limit size of the payload (global configuration)
	            // websocket route
	            .service(web::resource("/router/").route(web::get().to(ws_index)))
	            // static files
	            .service(fs::Files::new("/", &HTML_FOLDER.to_string()).index_file(&FILE_INDEX.to_string()))
	            // default
	            .default_service(
	                // 404 for GET request
	                web::resource("")
	                    .route(web::get().to(p404))
	                    // all requests that are not `GET`
	                    .route(
	                        web::route()
	                            .guard(guard::Not(guard::Get()))
	                            .to(HttpResponse::MethodNotAllowed),
	                    ),
	            )
            });

        // Create the server url from the given configuration
        let url = Url::parse(&config.server.url)?;

        // Bind the address
        let addrs = Self::url_to_socket_addrs(&url)?;
        if url.scheme() == "https" {
            server
                .bind_openssl(addrs.as_slice(), Self::build_tls(&config)?)?
                .run();
        } else {
            server.bind(addrs.as_slice())?.run();
        }

        Ok(Server {
            config: config.to_owned(),
            runner,
            url,
        })
    }

    /// Start the server
    pub fn start(self) -> Fallible<()> {
        // Start the actual main server
        self.runner.run()?;
        Ok(())
    }

    /// Build an SslAcceptorBuilder from a config
    fn build_tls(config: &Config) -> Fallible<SslAcceptorBuilder> {
        let mut tls_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
        tls_builder.set_private_key_file(&config.server.key, SslFiletype::PEM)?;
        tls_builder.set_certificate_chain_file(&config.server.cert)?;
        Ok(tls_builder)
    }

    /// Convert an `Url` to a vector of `SocketAddr`
    pub fn url_to_socket_addrs(url: &Url) -> Fallible<Vec<SocketAddr>> {
        let host = url
            .host()
            .ok_or_else(|| format_err!("No host name in the URL"))?;
        let port = url
            .port_or_known_default()
            .ok_or_else(|| format_err!("No port number in the URL"))?;
        let addrs;
        let addr;
        Ok(match host {
            url::Host::Domain(domain) => {
                addrs = (domain, port).to_socket_addrs()?;
                addrs.as_slice().to_owned()
            }
            url::Host::Ipv4(ip) => {
                addr = (ip, port).into();
                from_ref(&addr).to_owned()
            }
            url::Host::Ipv6(ip) => {
                addr = (ip, port).into();
                from_ref(&addr).to_owned()
            }
        })
    }
}
