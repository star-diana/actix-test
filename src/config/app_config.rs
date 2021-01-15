use dotenv;

pub struct ApplicationConfig {
    // actix 绑定的 host
    pub app_bind_host: String,
    // actix 绑定的端口
    pub app_port: String,
    // 应用日志级别
    pub rust_log: String,
    // 数据库类型
    pub db_type: String,
    // 数据库用户名
    pub db_username: String,
    // 数据库密码
    pub db_password: String,
    // 数据库 host
    pub db_host: String,
    // 数据库端口
    pub db_port: String,
    // 数据库名
    pub db_name: String,
    //数据库连接 query 参数
    pub db_query_str: String,
    // 签名 token 的 secret
    pub token_secret: String,
    // 签名 token 的 Issuer
    pub token_issuer: String,
}

// 默认配置
impl Default for ApplicationConfig {
    fn default() -> Self {
        Self {
            app_bind_host: dotenv::var("APP_BIND_HOST").unwrap_or(String::from("0.0.0.0")),
            app_port: dotenv::var("APP_PORT").unwrap_or(String::from("8080")),
            rust_log: dotenv::var("RUST_LOG").unwrap_or(String::from("")),
            db_type: dotenv::var("DB_TYPE").unwrap_or(String::from("mysql")),
            db_username: dotenv::var("DB_USERNAME").unwrap_or(String::from("")),
            db_password: dotenv::var("DB_PASSWORD").unwrap_or(String::from("")),
            db_host: dotenv::var("DB_HOST").unwrap_or(String::from("localhost")),
            db_port: dotenv::var("DB_PORT").unwrap_or(String::from("3306")),
            db_name: dotenv::var("DB_NAME").unwrap_or(String::from("")),
            db_query_str: dotenv::var("DB_QUERY_STR").unwrap_or(String::from("")),
            token_secret: dotenv::var("TOKEN_SECRET").unwrap_or(String::from("")),
            token_issuer: dotenv::var("TOKEN_ISSUER").unwrap_or(String::from("")),
        }
    }
}
