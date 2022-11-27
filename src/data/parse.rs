use std::collections::HashMap;
use log::debug;
use scraper::{Html, Selector, ElementRef};
use crate::{Content, err_panic, get_text};

#[derive(Debug)]
pub struct UntisParserResult {
    pub current: Content,
    pub upcoming: Content 
}

pub struct UntisParser {
    pub document: String
}

// see line 19
type TableVersions = (Vec<usize>, Vec<usize>);

impl UntisParser {
    // For some fucking, ungodly reason are multiple pages being created when there are too many
    // rows in one table. (Just why?! (╯°□°)╯︵ ┻━┻ We have unlimited vertical space!!). So we (the poor developer :C)
    // have to write some ~~stupid~~ **inconvenient** workaround:
    // First we are looping over all "center" elements and see if they have a ".mon_title" in them and
    // determine two date strings that represent the current and upcoming plan version.
    // Now the fun begins: We check if and what kind of datestring
    // (".mon_title") the center element has have. Then depending on what datestring is present, the index of the
    // center element is being inserted into the current or upcoming index array.
    //
    // Now whenever we loop over the center elements of the page we know exactly where the content
    // belongs - or if it's just an advert link to the untis website. (:ThisIsFine:)
    //
    // Honestly, this is the most unpleasing parsing experience I've ever had. (Could be a lot worse though,
    // at least there are some classnames ready for use :shrug:).
    fn parse_table_versions(&self, document: &Html) -> TableVersions {
        let mut current_center_index: Vec<usize> = vec![];
        let mut upcoming_center_index: Vec<usize> = vec![];

        let mut current_date: String = "".to_string();
        let mut upcoming_date: String = "".to_string();

        let center_selector = Selector::parse("center").unwrap();

        for (center_index, center_element) in document.select(&center_selector).enumerate() {
            let title_selector = Selector::parse(".mon_title").unwrap();

            for title in center_element.select(&title_selector) {
                let text = get_text(&title);
                // 25.11.2022 Freitag, Woche B (Seite 1 / 2) -> get first part
                let text = text.split(" ").collect::<Vec<&str>>()[0].to_owned();

                if upcoming_date == "" && current_date == "" {
                    current_date = text.clone();
                }  else if text != current_date && upcoming_date == "" {
                    upcoming_date = text.clone();
                }

                if text == current_date {
                    current_center_index.push(center_index);
                } else if text == upcoming_date {
                    upcoming_center_index.push(center_index);
                } else {
                    log::error!("Could not match date string -> unable to identify plan version.");
                }
            }
        }

        println!("{}, {}", current_date, upcoming_date);
        println!("{:?}, {:?}", current_center_index, upcoming_center_index);

        (current_center_index, upcoming_center_index)
    }

    fn parse_date(&self, document: &Html, versions: &TableVersions) -> (String, String) {
        let center_selector = Selector::parse("center").unwrap();
        let date_selector = Selector::parse("div.mon_title").unwrap();

        let mut current_date = "".to_owned();
        let mut upcoming_date = "".to_owned();

        for (center_index, center_element) in document.select(&center_selector).enumerate() {
            for date_item in center_element.select(&date_selector) {
                let date = get_text(&date_item);
                let date = date.split(" ").collect::<Vec<&str>>()[0].to_owned();

                if versions.0.contains(&center_index) {
                    current_date = date;
                } else if versions.1.contains(&center_index) {
                    upcoming_date = date;
                } else {
                    todo!("error")
                }
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

        let table_versions = self.parse_table_versions(&document);
        let date = self.parse_date(&document, &table_versions);
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
