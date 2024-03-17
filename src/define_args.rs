use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "shellDeep", version = "0.1.0", about = "A simple shell tool for translate text and file")]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[arg(name="text",short, long, default_value = "", help="The text to translate")]
    pub(crate) text: String,
    #[arg(name="source language",short, long, default_value = "AUTO", help="Language of the text to be translated. Options currently available:

AR - Arabic [1]
BG - Bulgarian
CS - Czech
DA - Danish
DE - German
EL - Greek
EN - English
ES - Spanish
ET - Estonian
FI - Finnish
FR - French
HU - Hungarian
ID - Indonesian
IT - Italian
JA - Japanese
KO - Korean
LT - Lithuanian
LV - Latvian
NB - Norwegian (Bokmål)
NL - Dutch
PL - Polish
PT - Portuguese (all Portuguese varieties mixed)
RO - Romanian
RU - Russian
SK - Slovak
SL - Slovenian
SV - Swedish
TR - Turkish
UK - Ukrainian
ZH - Chinese
If this parameter is omitted, the API will attempt to detect the language of the text and translate it.

[1] Please note that Arabic has not yet been added to the /languages endpoint because it does not yet support document translation;
only text translation is supported for Arabic at this time. When document translation support is added for Arabic,
we will a) remove this note and b) add Arabic to the /languages endpoint.")]
    pub(crate) sorce_language: String,
    #[arg(name="context",short, long, default_value = "", help="The context parameter makes it possible to include additional context that can influence a translation but is not translated itself.
This additional context can potentially improve translation quality when translating short,
low-context source texts such as product names on an e-commerce website,
article headlines on a news website, or UI elements.")]
    pub(crate) context: String,
    #[arg(name="preserve formatting",short, long, help="Sets whether the translation engine should respect the original formatting, even if it would usually correct some aspects.

The formatting aspects affected by this setting include:

Punctuation at the beginning and end of the sentence
Upper/lower case at the beginning of the sentence")]
    pub(crate) preserve_formatting: bool,
    #[arg(name="formality",short, long, default_value = "", help="Sets whether the translated text should lean towards formal or informal language.

This feature currently only works for target languages
DE (German),
FR (French),
IT (Italian),
ES (Spanish),
NL (Dutch),
PL (Polish),
PT-BR and PT-PT (Portuguese),
JA (Japanese),
and RU (Russian).
Learn more about the plain/polite feature for Japanese here.
Setting this parameter with a target language that does not support formality will fail, unless one of the prefer_... options are used. Possible options are:

default (default)
more - for a more formal language
less - for a more informal language
prefer_more - for a more formal language if available, otherwise fallback to default formality
prefer_less - for a more informal language if available, otherwise fallback to default formality")]
    pub(crate) formality: String,
    #[arg(name="output language",short, long, default_value = "EN", help="The language into which the text should be translated. Options currently available:
AR - Arabic [1]
BG - Bulgarian
CS - Czech
DA - Danish
DE - German
EL - Greek
EN - English (unspecified variant for backward compatibility; please select EN-GB or EN-US instead)
EN-GB - English (British)
EN-US - English (American)
ES - Spanish
ET - Estonian
FI - Finnish
FR - French
HU - Hungarian
ID - Indonesian
IT - Italian
JA - Japanese
KO - Korean
LT - Lithuanian
LV - Latvian
NB - Norwegian (Bokmål)
NL - Dutch
PL - Polish
PT - Portuguese (unspecified variant for backward compatibility; please select PT-BR or PT-PT instead)
PT-BR - Portuguese (Brazilian)
PT-PT - Portuguese (all Portuguese varieties excluding Brazilian Portuguese)
RO - Romanian
RU - Russian
SK - Slovak
SL - Slovenian
SV - Swedish
TR - Turkish
UK - Ukrainian
ZH - Chinese (simplified)
[1] Please note that Arabic has not yet been added to the /languages endpoint because it does not yet support document translation;
only text translation is supported for Arabic at this time. When document translation support is added for Arabic, we will a)
remove this note and b) add Arabic to the /languages endpoint.")]

    pub(crate) language_output: String,
    #[arg(name="init",short, long, help="Configure the tool",)]
    pub(crate) is_config: bool,
    #[arg(name="key",short, long, default_value = "", help="The configuration api key")]
    pub(crate) key: String,
}