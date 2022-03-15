use tokio_postgres::{Connection, Client, Error, Socket, Statement};
use tokio_postgres::tls::{NoTls, NoTlsStream};
use tokio::net::TcpStream;
use tokio_postgres::types::ToSql;
use tokio_postgres::row::Row;
use std::collections::HashMap;

pub struct DbItf {
    client: Client,
    prepared_queries: HashMap<String, Statement>
}

impl DbItf {
    pub async fn new() -> DbItf {
        let (client, connection) = tokio_postgres::connect("postgresql://server:server@localhost:5432/thermostat", NoTls).await.unwrap();

        let mut dbitf = DbItf {
            client,
            prepared_queries: HashMap::new()
        };
        dbitf.prepare_from_file("SELECT 1").await;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        return dbitf;
    }

    async fn prepare_from_file(&mut self, filename: &str) {
        self.prepared_queries.insert(String::from("a"), self.client.prepare(filename).await.unwrap());
    }

    pub async fn query(
        &self,
        name: &str,
        params: &[&(dyn ToSql + Sync)]
    ) -> Result<Vec<Row>, Error> {
        return self.client.query(&self.prepared_queries[name], params).await;
    }
}

