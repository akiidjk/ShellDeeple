mod define_args;

use std::fs::File;
use std::{env, fs};
use std::io::{Write};

use reqwest::{Client, Response};
use clap::Parser;
use regex::Regex;
use serde_json::Value;
use anyhow::{Result, Context};
use colored::Colorize;

static URL_API: &str = "https://api-free.deepl.com/v2/translate";

fn create_params<'a>(text: &'a str,language_input:&'a str,language_output:&'a str, context:&'a str, preserve_formatting: bool, formality:&'a str) -> Vec<(&'a str, &'a str)>{
    let mut params = vec![
        ("text", text),
        ("target_lang", language_output),
    ];

    if language_input != "AUTO" {
        params.push(("source_lang", language_input),)
    }

    if !context.is_empty(){
        params.push(("context",context))
    }

    if preserve_formatting {
        params.push(("preserve_formatting","true"))
    }

    if !formality.is_empty() {
        params.push(("formality",formality))
    }

    return params
}

fn display_generic_error(){
    log::error!("An error occurred during the translation ")
}

fn read_key() -> std::io::Result<String> {
    let path:&str = get_path();
    let file_path = format!("{}/.key-deeple", path);
    fs::read_to_string(file_path)
}

async fn get_translation(params: Vec<(&str, &str)>) -> Result<()> {
        let client = Client::new();

        let key = read_key().expect("Error in the read of string");
        let auth_header = format!("DeepL-Auth-Key {}", key.trim());

        log::debug!("{}",key);
        let response: Response = client.post(URL_API)
            .header("Authorization",auth_header)
            .form(&params)
            .send()
            .await?;

        log::debug!("Status: {}", response.status());
        if response.status().as_str() == "403" {
            log::error!("The key is not valid sorry");
        }else{
            let resp_text = response.text().await.context("Error during the read of request")?;
            log::debug!("Body: {}", resp_text);

            let parsed_response: Value = serde_json::from_str(&resp_text).context("Error during the parsing of json")?;

            if let Some(message_value) = parsed_response.get("message").and_then(serde_json::Value::as_str) {
                log::error!("Sorry, an error occurred: {:?}", message_value);
            } else {
                if let Some(Value::Array(array)) = parsed_response.get("translations") {
                    if let Some(Value::Object(obj)) = array.get(0) {
                        if let Some(Value::String(text)) = obj.get("text") {
                            log::info!("The translated text is: '{}'", text.purple())
                        } else {
                            display_generic_error();
                        }
                    } else {
                        display_generic_error();
                    }
                } else {
                    display_generic_error();
                }
            }
        }
        Ok(())
    }


fn get_path() -> &'static str {
    return if cfg!(target_os = "windows") {
        "C:\\ProgramData\\shellDeeple"
    } else {
        "/etc/shellDeeple"
    }
}
fn config(api_key: &str){

    let regex_key: Regex = Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}:fx$").unwrap();

    if api_key.is_empty() {
        log::error!("You need to specify the key of api");
        return
    }else if !regex_key.is_match(api_key) {
        log::error!("The key insert is not a valid key");
        return
    }

    let path:&str = get_path();

    fs::create_dir_all(path).expect("Error creating directory");

    let file_path = format!("{}/.key-deeple", path);
    let mut file = File::create(file_path)
        .expect("Error in the creation of config key file");

    file.write_all(&*api_key.as_bytes())
        .expect("Error in the writing of config key file");
}



#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let args = define_args::Args::parse();

    //Params tranlations
    let text: String = args.text;
    let language_input:String = args.sorce_language.to_uppercase();
    let language_output:String = args.language_output.to_uppercase();
    let context:String = args.context;
    let preserve_formatting:bool = args.preserve_formatting;
    let formality:String = args.formality;

    //Configuration
    let is_config:bool = args.is_config;
    let api_key:String = args.key;

    log::debug!("Input: {}",text);
    log::debug!("Source lang: {}",language_input);
    log::debug!("Target lang: {}",language_output);
    log::debug!("Context: {}",context);
    log::debug!("Preserve formatting: {}",preserve_formatting);
    log::debug!("Formality; {}",formality);
    log::debug!("To config: {}",is_config);
    log::debug!("Key: {}",api_key);

    if is_config {
        config(&api_key);
    } else {
        if text.is_empty() {
            log::error!("You need to specify a text to translate");
        }else {
            let params:Vec<(&str,&str)> = create_params(&text, &language_input, &language_output, &context, preserve_formatting, &formality);
            get_translation(params).await.expect("Error occurred");}
    }

}

// log::error!("Questo è un messaggio di errore");
// log::warn!("Questo è un messaggio di avviso");
// log::info!("Questo è un messaggio informativo");
// log::debug!("Questo è un messaggio di debug");
// log::trace!("Questo è un messaggio di trace");
