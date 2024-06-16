use actix_web::{
    web,
    HttpResponse,
    Responder,
    HttpRequest
};
use sqlx::{ SqlitePool, prelude::FromRow };
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Post {
    id: String,
    low_res_url: String,
    high_res_url: String,
    caption: String,
    permalink: String,
    timestamp: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GroupedRecipe {
    date: String,
    posts: Vec<Post>
}

#[derive(Deserialize)]
pub struct QueryParams {
    #[serde(default)]
    limit: Option<i64>,
    #[serde(default)]
    page: Option<usize>
}

#[derive(Serialize, Deserialize)]
pub struct PaginatedResult {
    total_pages: usize,
    current_page: usize,
    recipes: Vec<GroupedRecipe>
}


pub async fn get_data(
    pool: web::Data<SqlitePool>,
    req: HttpRequest,
    params: web::Query<QueryParams>
) -> impl Responder {

    let auth_header = req.headers().get("Authorization");

    if let Some(header_value) = auth_header {
        if let Ok(header_str) = header_value.to_str() {

            let access_token = header_str.trim();
            let expected_token = "0e89498d5b964f4aa2063ab28eb23a45";

            if !access_token.is_empty() && access_token == expected_token {

                match (params.limit, params.page) {
                    (Some(limit), _) if limit > 0 => {
                        let query = format!("SELECT * FROM feed LIMIT {}", limit);
        
                        let result = sqlx::query_as::<_, Post>(&query)
                            .fetch_all(pool.get_ref())
                            .await;
        
                        return match result {
                            Ok(data) => HttpResponse::Ok().json(data),
                            Err(_) => HttpResponse::InternalServerError().finish(),
                        };
                    }
                    (_, Some(page)) if page > 0 => {
                        let all_posts = sqlx::query_as::<_, Post>("SELECT * FROM feed")
                            .fetch_all(pool.get_ref())
                            .await;
        
                        match all_posts {
                            Ok(posts) => {
                                let posts_per_page = 5;
                                let data = format_data(posts, posts_per_page, page);
                                return HttpResponse::Ok().json(data);
                            },
                            Err(_) => return HttpResponse::InternalServerError().finish()
                        }
                    }
                    _ => return HttpResponse::BadRequest().finish()
                }

            }

        }
    }

    HttpResponse::Unauthorized().finish()

}


fn format_data(posts: Vec<Post>, posts_per_page: usize, page: usize) -> PaginatedResult {
    
    let mut grouped_posts: std::collections::HashMap<String, Vec<Post>> = std::collections::HashMap::new();
    
    for post in &posts {
        let date = extract_date(&post.timestamp);
        grouped_posts.entry(date).or_insert_with(Vec::new).push(post.clone());
    }

    let mut paginated_results: Vec<GroupedRecipe> = grouped_posts
        .into_iter()
        .map(|(date, posts)| GroupedRecipe { date, posts })
        .collect();

    if paginated_results.is_empty() { return PaginatedResult { total_pages: 1, current_page: 1, posts: Vec::new() } };
    let cloned = paginated_results.clone();

    paginated_results.sort_by(|a, b| b.date.cmp(&a.date));

    let mut last_page_start_index = (paginated_results.len() - 1) * posts_per_page;
    let mut start_index = std::cmp::min(last_page_start_index, (page - 1) * posts_per_page);
    let total_pages = calculate_total_pages(cloned.len(), posts_per_page);

    if start_index >= total_pages * posts_per_page {
        last_page_start_index = (total_pages - 1) * posts_per_page;
        start_index = std::cmp::max(0, last_page_start_index);
    }

    let posts_result = paginated_results.into_iter().skip(start_index).take(posts_per_page).collect();
    PaginatedResult {
        total_pages,
        current_page: std::cmp::min(total_pages, page),
        posts: posts_result
    }

}

fn extract_date(timestamp: &str) -> String {
    timestamp.split('T').next().unwrap_or_default().to_string()
}

fn calculate_total_pages(total_posts: usize, posts_per_page: usize) -> usize {
    (total_posts + posts_per_page - 1) / posts_per_page
}