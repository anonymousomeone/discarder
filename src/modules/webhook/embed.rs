#![allow(dead_code)]

// json hell
use serde::{ser::{Serialize, Serializer, SerializeStruct}, Serialize as Ser};
// not anymore

#[derive(Clone)]
pub struct EmbedBuilder {
    title: Option<String>,
    description: Option<String>,
    color: Option<String>,
    author: Option<EmbedAuthorData>,
    fields: Option<Vec<APIEmbedField>>,
}

// serde hell...
impl Serialize for EmbedBuilder {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("EmbedBuilder", 5)?;

        match &self.title {
            Some(_) =>  state.serialize_field("title", &self.title)?,
            None => {}
        }

        match &self.description {
            Some(_) => state.serialize_field("description", &self.description)?,
            None => {}
        }

        match &self.color {
            Some(_) => state.serialize_field("color", &self.color)?,
            None => {}
        }

        match &self.author {
            Some(_) =>  state.serialize_field("author", &self.author)?,
            None => {}
        }

        match &self.fields {
            Some(_) => state.serialize_field("fields", &self.fields)?,
            None => {}
        }

        state.end()
    }
}

impl EmbedBuilder {
    pub fn new() -> EmbedBuilder {
        EmbedBuilder {
            title: None,
            description: None,
            color: None,
            author: None,
            fields: None,
        }
    }

    pub fn set_title(mut self, title: String) -> EmbedBuilder {
        self.title = Some(title);
        self
    }

    pub fn set_description(mut self, description: String) -> EmbedBuilder {
        self.description = Some(description);
        self
    }

    pub fn set_color(mut self, color: String) -> EmbedBuilder {
        self.color = Some(color);
        self
    }

    pub fn add_field(mut self, name: String, value: String, inline: Option<bool>) -> EmbedBuilder {
        let field = APIEmbedField::new(name, value, inline);
        
        match self.fields {
            Some(_) => self.fields.as_mut().expect("guh").push(field),
            None => self.fields = Some(vec![field])
        }
        self
    }

    pub fn set_author(mut self, name: String, url: Option<String>, icon_url: Option<String>) -> EmbedBuilder {
        let author = EmbedAuthorData::new(name, url, icon_url);
        
        self.author = Some(author);

        self
    }

    pub fn build(self) -> Embed {
        let embed = Embed::new(self);

        embed
    }

}

#[derive(Ser, Clone, Debug)]
pub struct Embed {
    title: Option<String>,
    description: Option<String>,
    color: Option<String>,
    author: Option<EmbedAuthorData>,
    fields: Option<Vec<APIEmbedField>>,
}

impl Embed {
    fn new(builder: EmbedBuilder) -> Embed {
        Embed {
            title: builder.title,
            description: builder.description,
            color: builder.color,
            author: builder.author,
            fields: builder.fields
        }
    }
}

// https://discord.js.org/#/docs/discord.js/main/typedef/EmbedAuthorData
#[derive(Ser, Clone, Debug)]
struct EmbedAuthorData {
    name: String,
    url: Option<String>,
    icon_url: Option<String>
}

impl EmbedAuthorData {
    fn new(name: String, url: Option<String>, icon_url: Option<String>) -> EmbedAuthorData {
        EmbedAuthorData {
            name,
            url,
            icon_url
        }
    }
}

// https://discord-api-types.dev/api/discord-api-types-v10/interface/APIEmbedField
#[derive(Ser, Debug, Clone)]
struct APIEmbedField {
    name: String,
    value: String,
    inline: Option<bool>
}

impl APIEmbedField {
    fn new(name: String, value: String, inline: Option<bool>) -> APIEmbedField {
        APIEmbedField {
            name,
            value,
            inline
        }
    }
}