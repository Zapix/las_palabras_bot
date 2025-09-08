pub const DEFAULT_PAGE: u64 = 20;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[allow(dead_code)]
pub struct Pagination<T>
where
    T: serde::Serialize + utoipa::ToSchema,
{
    pub items: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
}

#[derive(Debug, Clone, serde::Deserialize, utoipa::ToSchema)]
pub struct PageQuery {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}
