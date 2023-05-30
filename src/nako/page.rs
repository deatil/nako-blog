use serde::{
    Serialize, 
    Deserialize,
};
use url::{
    Url,
    ParseError,
};

#[derive(Serialize, Deserialize)]
pub struct Pagination<T> {
    pub page: usize,
    pub per_page: usize,
    pub total: usize,
    pub items: Vec<T>,
}

/// 分页
impl<T> Pagination<T> {
    pub fn new(items: Vec<T>, page: usize, per_page: usize) -> Self {
        let total = items.len();
        let start_index = (page - 1) * per_page;
        let end_index = start_index + per_page;
        let items: Vec<T> = items.into_iter().skip(start_index).take(per_page).collect();

        Self {
            page,
            per_page,
            total,
            items,
        }
    }

    pub fn total_pages(&self) -> usize {
        (self.total + self.per_page - 1) / self.per_page
    }

    pub fn has_prev(&self) -> bool {
        self.page > 1
    }

    pub fn has_next(&self) -> bool {
        self.page < self.total_pages()
    }

    pub fn prev_page(&self) -> Option<usize> {
        if self.has_prev() {
            Some(self.page - 1)
        } else {
            None
        }
    }

    pub fn next_page(&self) -> Option<usize> {
        if self.has_next() {
            Some(self.page + 1)
        } else {
            None
        }
    }

    /// 生成链接
    pub fn page_links(&self, base_url_str: &str) -> String {
        let parsed_url = self.build_base_url(base_url_str);
        if let Err(_) = parsed_url {
            return base_url_str.to_string();
        }

        let base_url = parsed_url.unwrap();

        let mut links = Vec::new();
        let total_pages = self.total_pages();

        links.push("<ul class=\"pagination\">".to_string());
        links.push(self.build_prev_link(&base_url, self.has_prev(), self.page - 1));
        links.push(self.build_page_number_links(&base_url, self.page, total_pages));
        links.push(self.build_next_link(&base_url, self.has_next(), self.page + 1, total_pages));
        links.push("</ul>".to_string());

        links.join("")
    }

    fn build_base_url(&self, base_url_str: &str) -> Result<Url, ParseError> {
        let mut base_url = Url::parse(base_url_str)?;

        let base_url_clone = base_url.clone();

        base_url.query_pairs_mut().clear();

        for (key, value) in base_url_clone.query_pairs() {
            if key != "page" {
                base_url.query_pairs_mut().append_pair(&key, &value);
            }
        }

        Ok(base_url)
    }

    fn build_prev_link(&self, base_url: &Url, has_prev: bool, prev_page: usize) -> String {
        if has_prev {
            let prev_page_url = self.build_page_url(&base_url, prev_page);
            self.build_li_link(&prev_page_url, "Previous", "&laquo;")
        } else {
            self.build_disabled_li_link("Previous", "&laquo;")
        }
    }

    fn build_next_link(&self, base_url: &Url, has_next: bool, next_page: usize, total_pages: usize) -> String {
        if has_next {
            let next_page_url = self.build_page_url(&base_url, next_page);
            self.build_li_link(&next_page_url, "Next", "&raquo;")
        } else {
            self.build_disabled_li_link("Next", "&raquo;")
        }
    }

    fn build_page_number_links(&self, base_url: &Url, current_page: usize, total_pages: usize) -> String {
        let mut links = Vec::new();

        for i in (1 as i32).max(current_page as i32 - 4) as usize..=(total_pages as i32)
            .min(current_page as i32 + 4)
            as usize
        {
            let page_url = self.build_page_url(&base_url, i);
            if i == current_page {
                links.push(self.build_active_li_link(&i.to_string()))
            } else {
                links.push(self.build_li_link(&page_url, "", &i.to_string()));
            }
        }

        links.join("")
    }

    fn build_page_url(&self, url_builder: &Url, page: usize) -> String {
        let mut url_builder = url_builder.clone();
        url_builder.query_pairs_mut().append_pair("page", &page.to_string());
        url_builder.to_string()
    }

    fn build_li_link(&self, href: &str, label: &str, symbol: &str) -> String {
        format!(
            "<li class=\"page-item\"><a href=\"{}\" class=\"page-link\" aria-label=\"{}\"><span aria-hidden=\"true\">{}</span><span class=\"sr-only\">{}</span></a></li>",
            href,
            label,
            symbol,
            label
        )
    }

    fn build_disabled_li_link(&self, label: &str, symbol: &str) -> String {
        format!(
            "<li class=\"page-item disabled\"><a class=\"page-link\" aria-label=\"{}\"><span aria-hidden=\"true\">{}</span><span class=\"sr-only\">{}</span></a></li>",
            label,
            symbol,
            label
        )
    }

    fn build_active_li_link(&self, label: &str) -> String {
        format!(
            "<li class=\"page-item active\"><a class=\"page-link\">{}</a></li>",
            label
        )
    }
}


