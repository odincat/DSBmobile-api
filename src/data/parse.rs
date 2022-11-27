use std::collections::HashMap;
use log::debug;
use reqwest::Client;
use scraper::{Html, Selector};
use crate::{Content, err_panic, get_text};

#[derive(Debug)]
pub struct UntisParserResult {
    pub current: Content,
    pub upcoming: Content 
}

pub struct UntisParser {
    pub document: String
}

impl UntisParser {
    fn parse_date(&self, document: &Html) -> (String, String) {
        let date_selector = Selector::parse("div.mon_title").unwrap();

        let mut current_date = "".to_owned();
        let mut upcoming_date = "".to_owned();

        for (index, date_item) in document.select(&date_selector).enumerate() {
            let date_text = get_text(&date_item);
            let date = date_text.split(" ").collect::<Vec<&str>>()[0].to_owned();

            if index == 0 {
                current_date = date;
            } else if index == 1 {
                upcoming_date = date;
            }
        }

        (current_date, upcoming_date)
    }

    fn parse_weekday(&self, document: &Html) -> (String, String) {
        let date_selector = Selector::parse("div.mon_title").unwrap();

        let mut current_weekday = "".to_owned();
        let mut upcoming_weekday = "".to_owned();

        for (index, weekday_item) in document.select(&date_selector).enumerate() {
            let weekday_text = get_text(&weekday_item);
            let weekday = weekday_text.split(" ").collect::<Vec<&str>>()[1].to_owned();

            if index == 0 {
                current_weekday = weekday;
            } else if index == 1 {
                upcoming_weekday = weekday;
            }
        }

        (current_weekday, upcoming_weekday)
    }

    fn parse_week_type(&self, document: &Html) -> (String, String) {
        let date_selector = Selector::parse("div.mon_title").unwrap();

        let mut current_week_type = "".to_owned();
        let mut upcoming_week_type = "".to_owned();

        for (index, date_item) in document.select(&date_selector).enumerate() {
            let date = get_text(&date_item);
            // "11.11.2022 Freitag, Woche B", get last character
            let week_type = &date[date.len()-1..date.len()];

            if index == 0 {
                current_week_type = week_type.to_owned();
            } else if index == 1 {
                upcoming_week_type = week_type.to_owned();
            }
        }

        (current_week_type, upcoming_week_type)
    }

    fn parse_news(&self, document: &Html) -> (Vec<String>, Vec<String>) {
        let news_table_selector = Selector::parse("table.info").unwrap();

        let mut current_news: Vec<String> = vec![];
        let mut upcoming_news: Vec<String> = vec![];

        for (index, news_table) in document.select(&news_table_selector).enumerate() {
            let news_item_selector = Selector::parse("tr.info td.info b").unwrap();

            for news_item in news_table.select(&news_item_selector) {
                let news_item_text = news_item.text().collect::<Vec<_>>().join(" ");

                if index == 0 {
                    current_news.push(news_item_text);
                } else {
                    upcoming_news.push(news_item_text);
                }
            }
        }

        debug!("Current news: {:?}", current_news);
        debug!("Upcoming news: {:?}", upcoming_news);
        
        return (current_news, upcoming_news);
    }

    fn parse_content(&self, document: &Html) -> (Vec<HashMap<String, String>>, Vec<HashMap<String, String>>) {
        let content_table_selector = Selector::parse("table.mon_list").unwrap();

        let mut current_content: Vec<HashMap<String, String>> = vec![];
        let mut upcoming_content: Vec<HashMap<String, String>> = vec![];

        let mut table_headers: Vec<String> = vec![];

        for (table_index, content_table) in document.select(&content_table_selector).enumerate() {
            // Table headers
            let th_selector = Selector::parse("tr.list th").unwrap();
            for th in content_table.select(&th_selector) {
                let text = get_text(&th).to_lowercase();
                table_headers.push(text);
            }
        
            // Table content
            let mut items: Vec<HashMap<String, String>> = vec![];

            let tr_content_selector = Selector::parse("tr.list.odd, tr.list.even").unwrap();
            for tr in content_table.select(&tr_content_selector) {
                let td_selector = Selector::parse("td").unwrap();
                let mut item: HashMap<String, String> = HashMap::new();

                for (td_index, td) in tr.select(&td_selector).enumerate() {
                    let text = get_text(&td).to_lowercase();
                    item.insert(table_headers[td_index].to_owned(), text);
                }
                
                items.push(item);
            }

            if table_index == 0 {
                current_content = items;
            } else {
                upcoming_content = items;
            }
        }

        (current_content, upcoming_content)
    }

    pub async fn execute(&self) -> UntisParserResult {
        if self.document.len() == 0 {
            err_panic("HTML document in form of a string must be supplied");
        }

        let document = Html::parse_document(&self.document);

        let date = self.parse_date(&document);
        let weekday = self.parse_weekday(&document);
        let week_type = self.parse_week_type(&document);
        let news = self.parse_news(&document);
        let content = self.parse_content(&document);

        UntisParserResult {
            current: Content {
                content: content.0,
                date: date.0,
                news: news.0,
                week_type: week_type.0,
                weekday: weekday.0
            },
            upcoming: Content {
                content: content.1,
                date: date.1,
                news: news.1,
                week_type: week_type.1,
                weekday: weekday.1
            }
        }
    }
}
