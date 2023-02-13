use std::path::Path;

use sqlx_db_tester::TestPg;

// private none test functions
pub fn get_tdb() -> TestPg {
    dotenvy::from_filename(".env.test").ok();
    let server_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let migrations = Path::new("./migrations");
    TestPg::new(server_url, migrations)
}
