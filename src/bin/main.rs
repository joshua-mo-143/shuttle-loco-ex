use it::app::App;
use std::str::FromStr;
use tokio::net::TcpListener;
use loco_rs::{cli, boot::{create_app, start, StartMode, BootResult}, environment::Environment, app::{AppContext, Hooks}, db};
use migration::Migrator;
use sqlx::PgPool;
use sea_orm::DatabaseConnection;

struct LocoService { db: DatabaseConnection } 

#[shuttle_runtime::main]
async fn main(
#[shuttle_shared_db::Postgres] db: PgPool,
    ) -> Result<LocoService, shuttle_runtime::Error> {

    let db = sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(db);  // pg conn
                                                                             //
    Ok(LocoService{ db })
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for LocoService {
    async fn bind(
            mut self,
            addr: std::net::SocketAddr
        ) -> Result<(), shuttle_runtime::Error> {
        let start_mode = StartMode::ServerAndWorker;

    let environment = Environment::from_str("development")
        .unwrap();
    let config = environment.load().unwrap();

    let app_context = AppContext {
        environment,
        db: self.db,
        redis: None, 
        config,
        mailer: None,
    };

             let router = App::routes().to_router(app_context.clone()).unwrap();

        let tcplistener = TcpListener::bind(addr).await.unwrap();

        axum::serve(tcplistener, router).await.unwrap();
        

        Ok(())
    }
}
