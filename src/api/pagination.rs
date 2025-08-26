#[derive(Debug, Clone, serde::Serialize)]
#[allow(dead_code)]
pub struct Pagination<T>
where
    T: serde::Serialize,
{
    pub items: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PageQuery {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}
