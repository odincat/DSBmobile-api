use anyhow::{bail, Context, Result};
use std::collections::{BTreeMap, BTreeSet};
use scraper::{Html, Selector, ElementRef};
use crate::{get_text, PlanContent, SubstitutionPlanContent, ValuePair, some_or_bail};

#[derive(Debug)]
pub struct UntisParserResult {
    pub current: PlanContent,
    pub upcoming: PlanContent
}

struct CenterParseResult {
    date: ValuePair<DateParseResult>,
    news: ValuePair<Vec<String>>,
    content: ValuePair<Vec<BTreeMap<String, String>>>
}
impl CenterParseResult {
    pub fn default() -> CenterParseResult {
        CenterParseResult {
            date: (DateParseResult::default(), DateParseResult::default()),
            news: (vec![], vec![]),
            content: (vec![], vec![])
        }
    }
} 

struct DateParseResult {
    date: String,
    weekday: String,
    week_type: String
}
impl DateParseResult {
    pub fn default() -> DateParseResult {
        let default_string = "".to_string();
        DateParseResult { date: default_string.clone(), weekday: default_string.clone(), week_type: default_string.clone()  }
    }
}

// see rant
type TableVersions = (Vec<usize>, Vec<usize>);

pub struct UntisParser {
    pub document: String
}
impl UntisParser {
    // For some fucking, ungodly reason multiple pages are being created when there are too many
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
    // Honestly, this is the most unsatisfying parsing experience I've ever had. (Could be a lot worse though,
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

    fn parse_date(&self, center_element: &ElementRef) -> DateParseResult {
        let date_selector = Selector::parse("div.mon_title").unwrap();

        let date_element = some_or_bail!(center_element.select(&date_selector).next(), DateParseResult::default());

        let date_element_text = get_text(&date_element);

        let date = date_element_text.split(" ").collect::<Vec<&str>>()[0].to_owned();

        let weekday = date_element_text.split(" ").collect::<Vec<&str>>()[1].to_string();
        let weekday = weekday.split(",").collect::<Vec<&str>>()[0].to_string();

        let week_type = date_element_text.split(" ").collect::<Vec<&str>>()[3].to_string();

        DateParseResult {
            date,
            weekday,
            week_type
        }
    }

    fn parse_news(&self, center_element: &ElementRef) -> Vec<String> {
        let news_table_selector = Selector::parse("table.info").unwrap();

        // let news_table_element = match center_element.select(&news_table_selector).next() {
        //     Some(element) => element,
        //     // We can return anything, because it will not be further evaluated / used.
        //     None => return vec![]
        // };

        let news_table_element = some_or_bail!(center_element.select(&news_table_selector).next(), vec![]);

        let mut news: Vec<String> = vec![];

        let news_item_selector = Selector::parse("tbody tr.info td.info").unwrap();

        for news_item in news_table_element.select(&news_item_selector) {
            let news_item_text = news_item.text().collect::<Vec<_>>().join(" ");
            //TODO: remove newline characters
            news.push(news_item_text);
        }
        
        news
    }

    fn parse_content(&self, center_element: &ElementRef) -> Vec<BTreeMap<String, String>> {
        let content_table_selector = Selector::parse("table.mon_list").unwrap();

        let content_table_element = some_or_bail!(center_element.select(&content_table_selector).next(), vec![]);

        // Table headers
        let mut table_headers: Vec<String> = vec![];

        let th_selector = Selector::parse("tr.list > th.list").unwrap();

        for th in content_table_element.select(&th_selector) {
            let text = get_text(&th).to_lowercase();
            table_headers.push(text);
        }
    
        // Table content
        let mut items: Vec<BTreeMap<String, String>> = vec![];

        let tr_content_selector = Selector::parse("tr.list").unwrap();
        for tr in content_table_element.select(&tr_content_selector) {
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
        
        items
    }

    fn get_affected_classes(&self, content: &ValuePair<SubstitutionPlanContent>) -> ValuePair<Vec<String>> {
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

        (currently_affected, upcoming_affected)
    }

    fn center_section_parse(&self, document: &Html, versions: &TableVersions) -> Result<CenterParseResult> {
        let center_selector = Selector::parse("center").unwrap();

        let (current_version, upcoming_version) = versions;

        let mut parse_result = CenterParseResult::default();

        for (center_index, center_element) in document.select(&center_selector).enumerate() {
            if !current_version.contains(&center_index) && !upcoming_version.contains(&center_index) {
                continue;
            }

            let date = self.parse_date(&center_element);
            let news = self.parse_news(&center_element);
            let content = self.parse_content(&center_element);

            if current_version.contains(&center_index) {
                parse_result.date.0 = date;
                parse_result.news.0 = news;
                parse_result.content.0 = content;
            } else if upcoming_version.contains(&center_index) {
                parse_result.date.1 = date;
                parse_result.news.1 = news;
                parse_result.content.1 = content
            }
        }

        Ok(parse_result)
    }

    pub async fn execute(&self) -> Result<UntisParserResult> {
        if self.document.len() == 0 {
            bail!("HTML document in form of a string must be supplied");
        }

        let document = Html::parse_document(&self.document);

        let table_versions = self.parse_table_versions(&document);
        let CenterParseResult { date, news, content } = self.center_section_parse(&document, &table_versions)?;
        let affected_classes = self.get_affected_classes(&content);

        // recap: 0 is the current plan and 1 is the upcoming one
        Ok(UntisParserResult {
            current: PlanContent {
                content: content.0,
                date: date.0.date,
                news: news.0,
                week_type: date.0.week_type,
                weekday: date.0.weekday,
                affected_classes: affected_classes.0
            },
            upcoming: PlanContent {
                content: content.1,
                date: date.1.date,
                news: news.1,
                week_type: date.1.week_type,
                weekday: date.1.weekday,
                affected_classes: affected_classes.1
            }
        })
    }
}
