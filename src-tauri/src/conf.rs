use std::collections::HashMap;

use crate::utils;


pub const CONFIG_NAME: &str = "app.conf.json";

pub const DEFAULT_CONFIG: &str = r#"
{
    "autostart": true,
    "apps": [
        {
            "name": "ChatGPT",
            "icon": "https://chat.openai.com/favicon-32x32.png",
            "url": "https://chat.openai.com",
            "author": "OpenAI",
            "desc": "An artificial intelligence chatbot developed by OpenAI and built on top of OpenAI\"s GPT-3.5 and GPT-4 families of large language models (LLMs) "
        },
        {
            "name": "Poe",
            "icon": "https://poe.com/favicon.ico",
            "url": "https://poe.com",
            "author": "Poe",
            "desc": "A platform that lets people ask questions, get instant answers, and have back-and-forth conversations with AI."
        }
    ],
    "tools": [
        {
            "name": "Translator",
            "icon": "https://poe.com/favicon.ico",
            "prompt": "Translate this text from {langFrom} to {langTo}: {input}",
            "params": {
                "langFrom": "en",
                "langTo": "zh"
            },
            "desc": "Translator"
        }
    ]
}
        "#;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct App {
    pub name: String,
    pub icon: String,
    pub url: String,
    pub desc: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Tool {
    pub name: String,
    pub icon: String,
    pub prompt: String,
    pub params: HashMap<String, String>,
    pub desc: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Config {
    pub autostart: bool,
    pub apps: Vec<App>,
    pub tools: Vec<Tool>,
}

pub fn conf_path() -> std::path::PathBuf {
    utils::app_home().join(CONFIG_NAME)
}

pub fn init() {
    std::fs::create_dir_all(utils::app_home()).unwrap();
    let path = conf_path();
    if !path.exists() {
        std::fs::write(path, DEFAULT_CONFIG).unwrap();
    }
} 

pub fn load() -> Config {
    serde_json::from_str(&std::fs::read_to_string(conf_path()).unwrap()).unwrap()
}
pub fn save(c : &Config) {
    std::fs::write(conf_path(), serde_json::to_string_pretty(c).unwrap()).unwrap();
}

