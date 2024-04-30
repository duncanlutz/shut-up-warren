use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct MemeTemplate {
    pub id: String,
    pub lines: i32,
}

#[derive(Clone)]
pub struct Templates {
    pub templates: Vec<MemeTemplate>,
}

#[derive(Deserialize)]
pub struct MemeImageParams {
    pub id: String,
    pub lines: i32,
}

impl Templates {
    pub async fn new() -> Result<Templates, reqwest::Error> {
        let templates = MemeTemplate::get_all().await?;
        Ok(Templates { templates })
    }

    pub fn get_random(&self) -> &MemeTemplate {
        let index = rand::random::<usize>() % self.templates.len();
        &self.templates[index]
    }
}

impl MemeTemplate {
    pub async fn get_all() -> Result<Vec<MemeTemplate>, reqwest::Error> {
        let client = Client::new();
        let res = client
            .get("https://api.memegen.link/templates/")
            .send()
            .await?;
        let meme_templates: Vec<MemeTemplate> = res.json().await?;
        Ok(meme_templates)
    }

    pub async fn get_url(id: &str, lines: i32) -> Result<String, reqwest::Error> {
        let url = if lines == 1 {
            format!("https://api.memegen.link/images/{}/shut_up_warren.png", id)
        } else {
            let is_top = rand::random::<i32>() % 2 == 0;
            if is_top {
                format!("https://api.memegen.link/images/{}/shut_up_warren.png", id)
            } else {
                format!(
                    "https://api.memegen.link/images/{}/_/shut_up_warren.png",
                    id
                )
            }
        };

        Ok(url)
    }
}
