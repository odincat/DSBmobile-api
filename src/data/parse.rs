use reqwest::Client;
use tl::{VDom, Parser};
use crate::{Content, err_panic};

pub struct GenericPlanParserResult {
    pub current: Content,
    pub upcoming: Content 
}

pub struct GenericPlanParser {
    pub url: String
}

impl GenericPlanParser {
    fn parse_info(&self, dom: &VDom<'_>, parser: &Parser<'_>) -> Vec<String> {
        let res = dom.query_selector("table.info").expect("el not found").for_each(|el| {
            for el in el.get(parser).unwrap().children().iter() {
                
            }
        }); 

        vec!["".to_owned()]
    }

    pub async fn execute(&self) -> GenericPlanParserResult {
        if(self.url.len() == 0) {
            err_panic("URL must be supplied");
        }

        let client = Client::new();

        let response = client.get(self.url.clone()).send().await.unwrap().text().await.unwrap();
        let dom = tl::parse(&response, tl::ParserOptions::default()).unwrap();
        let parser = &dom.parser();

        let info = self.parse_info(&dom, &parser);

        GenericPlanParserResult { current: Content { info: vec!["hi".to_owned()], content: serde_json::Value::Null }, upcoming: Content { info: vec![], content: serde_json::Value::Null } }
    }
}