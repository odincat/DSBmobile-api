use anyhow::{bail, Result};
use std::collections::{BTreeMap, BTreeSet};
use scraper::{Html, Selector, ElementRef};
use crate::{get_text, Plan, Substitutions, ValuePair, some_or_bail};

#[derive(Debug)]
pub struct UntisOutput {
    pub current: Plan,
    pub upcoming: Plan
}

#[derive(Debug, PartialEq)]
struct DateParseOutput {
    date: String,
    weekday: String,
    week_type: String
}
impl DateParseOutput {
    pub fn default() -> DateParseOutput {
        DateParseOutput {
            date: "".to_string(),
            weekday: "".to_string(),
            week_type: "".to_string()
        }
    }
}

struct CenterOutput {
    date: ValuePair<DateParseOutput>,
    news: ValuePair<Vec<String>>,
    content: ValuePair<Vec<BTreeMap<String, String>>>
}
impl CenterOutput {
    pub fn default() -> CenterOutput {
        CenterOutput {
            date: (DateParseOutput::default(), DateParseOutput::default()),
            news: (vec![], vec![]),
            content: (vec![], vec![])
        }
    }
} 

// see rant
type TableVersions = ValuePair<Vec<usize>>;

pub struct UntisParser {
    pub document: String
}
impl UntisParser {
    // For future reference: This was me just spitting out my thoughts, because I didn't realize
    // that this was a thing. (Obviously caused a bug that took some time to fix). Don't take it seriously!
    //
    // For some ungodly reason multiple pages are being created when there are too many
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
    fn parse_table_versions(&self, document: &Html) -> Result<TableVersions> {
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
                    bail!("Unable to match plan date '{}' to '{}' and '{}'", &text, &current_date, &upcoming_date)
                }
            }
        }

        Ok((current_center_index, upcoming_center_index))
    }

    fn parse_date(&self, center_element: &ElementRef) -> DateParseOutput {
        let date_selector = Selector::parse("div.mon_title").unwrap();

        let date_element = some_or_bail!(center_element.select(&date_selector).next(), DateParseOutput::default());

        let date_element_text = get_text(&date_element);

        let date = date_element_text.split(" ").collect::<Vec<&str>>()[0].to_owned();

        let weekday = date_element_text.split(" ").collect::<Vec<&str>>()[1].to_string();
        let weekday = weekday.split(",").collect::<Vec<&str>>()[0].to_string();

        let week_type = date_element_text.split(" ").collect::<Vec<&str>>()[3].to_string();

        DateParseOutput {
            date,
            weekday,
            week_type
        }
    }

    fn parse_news(&self, center_element: &ElementRef) -> Vec<String> {
        let news_table_selector = Selector::parse("table.info").unwrap();

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

    fn get_affected_classes(&self, content: &ValuePair<Substitutions>) -> ValuePair<Vec<String>> {
        let (mut currently_affected, mut upcoming_affected) = (Vec::<String>::new(), Vec::<String>::new());
        let plan = vec![content.0.clone(), content.1.clone()];

        for (index, content_map) in plan.iter().enumerate() {
            let raw_classes: Vec<String> = content_map.iter().map(|item| {
                item.get("klasse(n)").unwrap().replace("(", "").replace(")", "").to_string()
            }).collect();

            let mut classes = BTreeSet::<String>::new();

            for class in raw_classes {
                for single_class in class.split(",") {
                    let single_class = single_class.trim().to_string();

                    if single_class.is_empty() {
                        continue;
                    }

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

    fn center_section_parse(&self, document: &Html, versions: &TableVersions) -> Result<CenterOutput> {
        let center_selector = Selector::parse("center").unwrap();

        let (current_version, upcoming_version) = versions;

        let mut parse_result = CenterOutput::default();

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
                parse_result.content.0.extend(content);
            } else if upcoming_version.contains(&center_index) {
                parse_result.date.1 = date;
                parse_result.news.1 = news;
                parse_result.content.1.extend(content);
            }
        }

        Ok(parse_result)
    }

    pub async fn execute(&self) -> Result<UntisOutput> {
        if self.document.is_empty() {
            bail!("HTML document in form of a string must be supplied");
        }

        let document = Html::parse_document(&self.document);

        let table_versions = self.parse_table_versions(&document)?;
        let CenterOutput { date, news, content } = self.center_section_parse(&document, &table_versions)?;
        let affected_classes = self.get_affected_classes(&content);

        // recap: 0 is the current plan and 1 is the upcoming one
        Ok(UntisOutput {
            current: Plan {
                content: content.0,
                date: date.0.date,
                news: news.0,
                week_type: date.0.week_type,
                weekday: date.0.weekday,
                affected_classes: affected_classes.0
            },
            upcoming: Plan {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_parser(test_file_path: &str) -> (UntisParser, String) {
        let file = std::fs::read_to_string(format!("tests/plans/{}", test_file_path)).unwrap();

        let parser = UntisParser {
            document: file.clone()
        };

        (parser, file)
    }

    #[tokio::test]
    async fn bail_on_empty_document() {
        let parser = UntisParser {
            document: "".to_string()
        };

        let result = parser.execute().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn date_parsing() {
        let (parser, file) = setup_parser("date.html");
        let element = Html::parse_document(&file);

        for (index, center_element) in element.select(&Selector::parse("center").unwrap()).enumerate() {
            let date = parser.parse_date(&center_element);

            match index {
                0 => assert_eq!(date, DateParseOutput {
                    date: "16.12.2022".to_string(),
                    weekday: "Freitag".to_string(),
                    week_type: "A".to_string()
                }),
                1 => assert_eq!(date, DateParseOutput {
                    date: "19.12.2022".to_string(),
                    weekday: "Montag".to_string(),
                    week_type: "B".to_string()
                }),
                2 => assert_eq!(date, DateParseOutput {
                    date: "15.12.2022".to_string(),
                    weekday: "Donnerstag".to_string(),
                    week_type: "A".to_string()
                }),
                _ => {}
            }

        }
    }

    #[tokio::test]
    async fn table_version_parsing() {
        let (parser, file) = setup_parser("full-1.html");
        let document = Html::parse_document(&file);

        let (current, upcoming) = parser.parse_table_versions(&document).unwrap();

        assert_eq!(current, vec![0, 2]);
        assert_eq!(upcoming, vec![4, 6]);

        let (parser, file) = setup_parser("full-2.html");
        let document = Html::parse_document(&file);

        let (current, upcoming) = parser.parse_table_versions(&document).unwrap();

        assert_eq!(current, vec![0, 2]);
        assert_eq!(upcoming, vec![4]);

        let (parser, file) = setup_parser("full-3.html");
        let document = Html::parse_document(&file);

        let (current, upcoming) = parser.parse_table_versions(&document).unwrap();

        assert_eq!(current, vec![0, 2]);
        assert_eq!(upcoming, vec![4]);
    }
}
