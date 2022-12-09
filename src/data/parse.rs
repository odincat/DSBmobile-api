use std::collections::{BTreeMap, BTreeSet};
use scraper::{Html, Selector};
use crate::{err_panic, get_text, PlanContent};

#[derive(Debug)]
pub struct UntisParserResult {
    pub current: PlanContent,
    pub upcoming: PlanContent
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
    // determine two date strings (from ".mon_title") that represent the current and upcoming plan version.
    // Now the fun begins: We check if and what kind of datestring
    // (".mon_title") the center element has. Then depending on what datestring is present, the index of the
    // center element is being inserted into the current or upcoming array.
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

                if current_date == "" && upcoming_date == "" {
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

    fn parse_weekday(&self, document: &Html, versions: &TableVersions) -> (String, String) {
        let center_selector = Selector::parse("center").unwrap();
        let date_selector = Selector::parse("div.mon_title").unwrap();

        let mut current_weekday = "".to_owned();
        let mut upcoming_weekday = "".to_owned();

        for (center_index, center_element) in document.select(&center_selector).enumerate() {
            for weekday_item in center_element.select(&date_selector) {
                let weekday = get_text(&weekday_item);
                let weekday = weekday.split(" ").collect::<Vec<&str>>()[1].to_string();
                let weekday = weekday.split(",").collect::<Vec<&str>>()[0].to_string();

                if versions.0.contains(&center_index) {
                    current_weekday = weekday;
                } else if versions.1.contains(&center_index) {
                    upcoming_weekday = weekday;
                } else {
                    todo!("error")
                }
            }
        }

        (current_weekday, upcoming_weekday)
    }

    fn parse_week_type(&self, document: &Html, versions: &TableVersions) -> (String, String) {
        let center_selector = Selector::parse("center").unwrap();
        let date_selector = Selector::parse("div.mon_title").unwrap();

        let mut current_week_type = "".to_owned();
        let mut upcoming_week_type = "".to_owned();

        for (center_index, center_element) in document.select(&center_selector).enumerate() {
            for date_item in center_element.select(&date_selector) {
                let week_type = get_text(&date_item);
                let week_type = week_type.split(" ").collect::<Vec<&str>>()[3].to_string();

                if versions.0.contains(&center_index) {
                    current_week_type = week_type.to_owned();
                } else if versions.1.contains(&center_index) {
                    upcoming_week_type = week_type.to_owned();
                } else {
                    todo!("error")
                }
            }
        }

        (current_week_type, upcoming_week_type)
    }

    fn parse_news(&self, document: &Html, versions: &TableVersions) -> (Vec<String>, Vec<String>) {
        let center_selector = Selector::parse("center").unwrap();
        let news_table_selector = Selector::parse("table.info").unwrap();

        let mut current_news: Vec<String> = vec![];
        let mut upcoming_news: Vec<String> = vec![];

        for (center_index, center_element) in document.select(&center_selector).enumerate() {
            for news_table in center_element.select(&news_table_selector) {
                let news_item_selector = Selector::parse("tbody tr.info td.info").unwrap();

                for news_item in news_table.select(&news_item_selector) {
                    let news_item_text = news_item.text().collect::<Vec<_>>().join(" ");

                    if versions.0.contains(&center_index) {
                        current_news.push(news_item_text);
                    } else if versions.1.contains(&center_index) {
                        upcoming_news.push(news_item_text);
                    } else {
                        todo!("error")
                    }
                }
            }
        }
        
        return (current_news, upcoming_news);
    }

    fn parse_content(&self, document: &Html, versions: &TableVersions) -> (Vec<BTreeMap<String, String>>, Vec<BTreeMap<String, String>>) {
        let center_selector = Selector::parse("center").unwrap();
        let content_table_selector = Selector::parse("table.mon_list").unwrap();

        let mut current_content: Vec<BTreeMap<String, String>> = vec![];
        let mut upcoming_content: Vec<BTreeMap<String, String>> = vec![];

        let mut table_headers: Vec<String> = vec![];

        for (center_index, center_element) in document.select(&center_selector).enumerate() {
            for content_table in center_element.select(&content_table_selector) {
                // Table headers
                let th_selector = Selector::parse("tr.list > th.list").unwrap();
                for th in content_table.select(&th_selector) {
                    let text = get_text(&th).to_lowercase();
                    table_headers.push(text);
                }
            
                // Table content
                let mut items: Vec<BTreeMap<String, String>> = vec![];

                let tr_content_selector = Selector::parse("tr.list").unwrap();
                for tr in content_table.select(&tr_content_selector) {
                    let td_selector = Selector::parse("td").unwrap();

                    let mut item: BTreeMap<String, String> = BTreeMap::new();

                    for (td_index, td) in tr.select(&td_selector).enumerate() {
                        let text = get_text(&td);

                        item.insert(table_headers[td_index].to_owned(), text);
                    }

                    if item.is_empty() {
                        continue;
                    }
                    items.push(item);
                }


                if versions.0.contains(&center_index) {
                    current_content.extend(items);
                } else if versions.1.contains(&center_index) {
                    upcoming_content.extend(items);
                } else {
                    todo!("error")
                }
            }
        }

        (current_content, upcoming_content)
    }

    fn get_affected_classes(&self, content: &(Vec<BTreeMap<String, String>>, Vec<BTreeMap<String, String>>)) -> (Vec<String>, Vec<String>) {
        let (mut currently_affected, mut upcoming_affected) = (Vec::<String>::new(), Vec::<String>::new());
        let plan_content = vec![content.0.clone(), content.1.clone()];

        for (index, content_map) in plan_content.iter().enumerate() {
            let raw_classes: Vec<String> = content_map.iter().map(|item| {
                item.get("klasse(n)").unwrap().to_string()
            }).collect();

            let mut classes = BTreeSet::<String>::new();

            for class in raw_classes {
                for single_class in class.split(",") {
                    let single_class = single_class.trim().to_string();

                    classes.insert(single_class);
                }
            }

            if index == 0 {
                currently_affected = classes.into_iter().collect();
            } else if index == 1 {
                upcoming_affected = classes.into_iter().collect();
            }
        }

        println!("{:?}", currently_affected);
        println!("{:?}", upcoming_affected);

        (currently_affected, upcoming_affected)
    }

    pub async fn execute(&self) -> UntisParserResult {
        if self.document.len() == 0 {
            err_panic("HTML document in form of a string must be supplied");
        }

        let document = Html::parse_document(&self.document);

        let table_versions = self.parse_table_versions(&document);
        let date = self.parse_date(&document, &table_versions);
        let weekday = self.parse_weekday(&document, &table_versions);
        let week_type = self.parse_week_type(&document, &table_versions);
        let news = self.parse_news(&document, &table_versions);
        let content = self.parse_content(&document, &table_versions);
        let affected_classes = self.get_affected_classes(&content);

        UntisParserResult {
            current: PlanContent{
                content: content.0,
                date: date.0,
                news: news.0,
                week_type: week_type.0,
                weekday: weekday.0,
                affected_classes: affected_classes.0
            },
            upcoming: PlanContent{
                content: content.1,
                date: date.1,
                news: news.1,
                week_type: week_type.1,
                weekday: weekday.1,
                affected_classes: affected_classes.1
            }
        }
    }
}
