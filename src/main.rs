#[allow(unused_labels)]
use color_eyre::eyre::Result;
use tracing::{info, Level, warn};
use tracing_subscriber::FmtSubscriber;
use poem::{listener::TcpListener, Route};
use poem_openapi::{
    param::Query,
    payload::PlainText,
    OpenApi,
    OpenApiService
};

mod argparser;

const HELPPARAMETER: &str = "--help";

//  The help text to display when --help is given
static HELP: &'static str = "\
Simple web server.\n\
--loglevel\t\tLog level to use. Defaults to Info.\n\
--help\t\tDisplay this help and exit.";

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("Hello, {}!", name)),
            None => PlainText("Hello!".to_string()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    match color_eyre::install() {
        Ok(_) => (),
        Err(e) => throw_fatal_error(format!("Error installing color eyre: {}", e).as_str()),
    };

    match parse_arguments() {
        Ok(level) => {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(level)
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
        }
        Err(e) => throw_fatal_error(format!("Error parsing arguments: {}", e).as_str()),
    };

    info!("starting up");

    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000/api");
    let ui = api_service.rapidoc();
    let app = Route::new().nest("/api", api_service).nest("/", ui);

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}

//  Throws an error and exits the program
fn throw_fatal_error(e: &str) {
    println!("Error: {}\r\n{}", &e, &HELP);
    std::process::exit(-1);
}

fn parse_arguments() -> Result<Level, String> {
    if argparser::get_parameter_switch_from_args(HELPPARAMETER) {
        println!("{}", HELP);
        std::process::exit(0);
    }

    return argparser::get_parameter_value_from_args::<Level>("--loglevel", "Invalid log level");
}