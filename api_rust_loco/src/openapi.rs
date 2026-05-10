use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::controllers::product::get_products_with_categories,
        crate::controllers::product::list,
        crate::controllers::product::get_one,
        crate::controllers::product::remove
    ),
    components(
        // We can add schemas here later if we want to explicitly define them
        // schemas(crate::models::products::ProductWithCategory)
    ),
    tags(
        (name = "Products", description = "Product management endpoints")
    )
)]
pub struct ApiDoc;
