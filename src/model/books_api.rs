use serde::{Deserialize, Serialize};

// region - BooksApiResponse
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BooksApiResponse {
    pub kind: Option<String>,
    pub id: Option<String>,
    pub etag: Option<String>,
    pub self_link: Option<String>,
    pub volume_info: Option<VolumeInfo>,
    pub layer_info: Option<LayerInfo>,
    pub user_info: Option<UserInfo>,
    pub sale_info: Option<SaleInfo>,
    pub access_info: Option<AccessInfo>,
    pub search_info: Option<SearchInfo>,
}

impl BooksApiResponse {
    pub fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    pub fn get_title(&self) -> String {
        match &self.volume_info {
            Some(volume_info) => match &volume_info.title {
                Some(title) => title.clone(),
                None => "".to_string(),
            },
            None => "".to_string(),
        }
    }

    pub fn get_authors(&self) -> Vec<String> {
        match &self.volume_info {
            Some(volume_info) => match &volume_info.authors {
                Some(authors) => authors.clone(),
                None => vec![],
            },
            None => vec![],
        }
    }

    pub fn get_publisher(&self) -> String {
        match &self.volume_info {
            Some(volume_info) => match &volume_info.publisher {
                Some(publisher) => publisher.clone(),
                None => "".to_string(),
            },
            None => "".to_string(),
        }
    }

    pub fn get_published_date(&self) -> String {
        match &self.volume_info {
            Some(volume_info) => match &volume_info.published_date {
                Some(published_date) => published_date.clone(),
                None => "".to_string(),
            },
            None => "".to_string(),
        }
    }

    pub fn get_description(&self) -> String {
        match &self.volume_info {
            Some(volume_info) => match &volume_info.description {
                Some(description) => description.clone(),
                None => "".to_string(),
            },
            None => "".to_string(),
        }
    }

    pub fn get_isbn10(&self) -> String {
        match &self.volume_info {
            Some(volume_info) => match &volume_info.industry_identifiers {
                Some(industry_identifiers) => {
                    for industry_identifier in industry_identifiers {
                        if industry_identifier.type_field == Some("ISBN_10".to_string()) {
                            return industry_identifier
                                .identifier
                                .clone()
                                .unwrap_or("".to_string());
                        }
                    }
                    "".to_string()
                }
                None => "".to_string(),
            },
            None => "".to_string(),
        }
    }

    pub fn get_isbn13(&self) -> String {
        match &self.volume_info {
            Some(volume_info) => match &volume_info.industry_identifiers {
                Some(industry_identifiers) => {
                    for industry_identifier in industry_identifiers {
                        if industry_identifier.type_field == Some("ISBN_13".to_string()) {
                            return industry_identifier
                                .identifier
                                .clone()
                                .unwrap_or("".to_string());
                        }
                    }
                    "".to_string()
                }
                None => "".to_string(),
            },
            None => "".to_string(),
        }
    }

    pub fn get_page_count(&self) -> i64 {
        match &self.volume_info {
            Some(volume_info) => match &volume_info.page_count {
                Some(page_count) => *page_count,
                None => 0,
            },
            None => 0,
        }
    }

    pub fn get_categories(&self) -> Vec<String> {
        match &self.volume_info {
            Some(volume_info) => match &volume_info.categories {
                Some(categories) => categories.clone(),
                None => vec![],
            },
            None => vec![],
        }
    }

    pub fn get_language(&self) -> String {
        match &self.volume_info {
            Some(volume_info) => match &volume_info.language {
                Some(language) => language.clone(),
                None => "".to_string(),
            },
            None => "".to_string(),
        }
    }

    pub fn get_cover(&self) -> String {
        match &self.volume_info {
            Some(volume_info) => match &volume_info.image_links {
                Some(image_links) => match &image_links.thumbnail {
                    Some(thumbnail) => thumbnail.clone(),
                    None => match &image_links.small_thumbnail {
                        Some(small_thumbnail) => small_thumbnail.clone(),
                        None => match &image_links.small {
                            Some(small) => small.clone(),
                            None => match &image_links.medium {
                                Some(medium) => medium.clone(),
                                None => match &image_links.large {
                                    Some(large) => large.clone(),
                                    None => match &image_links.extra_large {
                                        Some(extra_large) => extra_large.clone(),
                                        None => "".to_string(),
                                    },
                                },
                            },
                        },
                    },
                },
                None => "".to_string(),
            },
            None => "".to_string(),
        }
    }
}
// endregion - BooksApiResponse

// region - VolumeInfo
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeInfo {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub authors: Option<Vec<String>>,
    pub publisher: Option<String>,
    pub published_date: Option<String>,
    pub description: Option<String>,
    pub industry_identifiers: Option<Vec<IndustryIdentifier>>,
    pub page_count: Option<i64>,
    pub dimensions: Option<Dimensions>,
    pub print_type: Option<String>,
    pub main_category: Option<String>,
    pub categories: Option<Vec<String>>,
    pub average_rating: Option<f64>,
    pub ratings_count: Option<i64>,
    pub content_version: Option<String>,
    pub image_links: Option<ImageLinks>,
    pub language: Option<String>,
    pub preview_link: Option<String>,
    pub info_link: Option<String>,
    pub canonical_volume_link: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndustryIdentifier {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub identifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dimensions {
    pub height: Option<String>,
    pub width: Option<String>,
    pub thickness: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageLinks {
    pub small_thumbnail: Option<String>,
    pub thumbnail: Option<String>,
    pub small: Option<String>,
    pub medium: Option<String>,
    pub large: Option<String>,
    pub extra_large: Option<String>,
}
// endregion - VolumeInfo

// region - LayerInfo
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerInfo {
    pub layers: Option<Vec<Layer>>,
}
// endregion - LayerInfo

// region - Layer
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    pub layer_id: Option<String>,
    pub volume_annotations_version: Option<String>,
}
// endregion - Layer

// region - UserInfo
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub review: Option<String>,
    pub reading_position: Option<String>,
    pub is_purchased: Option<bool>,
    pub is_preordered: Option<bool>,
    pub updated: Option<String>,
}
// endregion - UserInfo

// region - SaleInfo
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaleInfo {
    pub country: Option<String>,
    pub saleability: Option<String>,
    pub on_sale_date: Option<String>,
    pub is_ebook: Option<bool>,
    pub list_price: Option<ListPrice>,
    pub retail_price: Option<RetailPrice>,
    pub buy_link: Option<String>,
}
// endregion - SaleInfo

// region - ListPrice
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPrice {
    pub amount: Option<f64>,
    pub currency_code: Option<String>,
}
// endregion - ListPrice

// region - RetailPrice
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetailPrice {
    pub amount: Option<f64>,
    pub currency_code: Option<String>,
}
// endregion - RetailPrice

// region - AccessInfo
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessInfo {
    pub country: Option<String>,
    pub viewability: Option<String>,
    pub embeddable: Option<bool>,
    pub public_domain: Option<bool>,
    pub text_to_speech_permission: Option<String>,
    pub epub: Option<Epub>,
    pub pdf: Option<Pdf>,
    pub web_reader_link: Option<String>,
    pub access_view_status: Option<String>,
    pub download_access: Option<DownloadAccess>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Epub {
    pub is_available: Option<bool>,
    pub download_link: Option<String>,
    pub acs_token_link: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pdf {
    pub is_available: Option<bool>,
    pub download_link: Option<String>,
    pub acs_token_link: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadAccess {
    pub kind: Option<String>,
    pub volume_id: Option<String>,
    pub restricted: Option<bool>,
    pub device_allowed: Option<bool>,
    pub just_acquired: Option<bool>,
    pub max_download_devices: Option<i64>,
    pub downloads_acquired: Option<i64>,
    pub nonce: Option<String>,
    pub source: Option<String>,
    pub reason_code: Option<String>,
    pub message: Option<String>,
    pub signature: Option<String>,
}
// endregion - AccessInfo

// region - SearchInfo
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchInfo {
    pub text_snippet: Option<String>,
}
// endregion - SearchInfo
