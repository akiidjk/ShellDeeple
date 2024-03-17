mod define_args;

use std::fs::File;
use std::io::Write;

use reqwest::{Client, Error, Response, header};
use clap::Parser;
use regex::Regex;
use serde::Serialize;
use serde_json::Value;
// use colored::Colorize;
use anyhow::{Result, Context};

//TODO USE THE COLORED


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

async fn get_translation(params: Vec<(&str, &str)>) -> Result<()> {
        let client = Client::new();
        let response: Response = client.post(URL_API)
            .header("Authorization", "DeepL-Auth-Key 104fa937-067d-ed70-11c3-7faa51a52fd8:fx")
            .form(&params)
            .send()
            .await?;

        println!("Status: {}", response.status());
        let resp_text = response.text().await.context("Errore durante la lettura della risposta")?;
        println!("Body: {}", resp_text);

        let parsed_response: Value = serde_json::from_str(&resp_text).context("Errore durante il parsing del JSON")?;

        if let Some(message_value) = parsed_response.get("message").and_then(serde_json::Value::as_str) {
            println!("Sorry, an error occurred: {:?}", message_value);
        } else {
            if let Some(Value::Array(array)) = parsed_response.get("translations") {
                if let Some(Value::Object(obj)) = array.get(0) {
                    if let Some(Value::String(text)) = obj.get("text") {
                        println!("The translated text is: {}", text);
                    } else {
                        println!("Il campo 'text' non è presente o non è una stringa");
                    }
                } else {
                    println!("Il primo elemento dell'array non è un oggetto");
                }
            } else {
                println!("La chiave non esiste o non è associata a un array");
            }
        }
        Ok(())
    }


fn config(api_key: &str){
    let regex_key: Regex = Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}:fx$").unwrap();

    if api_key == "" {
        println!("You need to specify the key of api");
        return
    }else if !regex_key.is_match(api_key) {
        println!("The key insert is not a valid key");
        return
    }

    let mut file = File::create("key.txt")
            .expect("Error in the creation of config key file");

    file.write_all(&*api_key.as_bytes())
        .expect("Error in the writing of config key file");
}



#[tokio::main]
async fn main() {
    println!("START");
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

    println!("Input: {}",text);
    println!("Source lang: {}",language_input);
    println!("Target lang: {}",language_output);
    println!("Context: {}",context);
    println!("Preserve formatting: {}",preserve_formatting);
    println!("Formality; {}",formality);
    println!("To config: {}",is_config);
    println!("Key: {}",api_key);

    if is_config {
        config(&api_key);
    } else {
        if text.is_empty() {
            println!("You need to specify a text to translate");
        }else {
            let params:Vec<(&str,&str)> = create_params(&text, &language_input, &language_output, &context, preserve_formatting, &formality);
            get_translation(params).await.expect("TODO: panic message");}
    }




}