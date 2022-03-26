pub mod temperature;
pub mod heater_timeslot;
pub mod setting;

use tokio_postgres::{Client, Error, Statement, Transaction};
use tokio_postgres::tls::{NoTls};
use tokio_postgres::types::ToSql;
use tokio_postgres::row::Row;
use std::collections::HashMap;
use std::fs;

static QUERY_PREFIX: &str = "sql/query/";

pub struct DbItf {
    client: Client,
    prepared_queries: HashMap<String, Statement>
}

pub struct DbTransaction<'a> {
    prepared_queries: &'a HashMap<String, Statement>,
    transaction: Transaction<'a>
}

impl DbItf {
    pub async fn new() -> DbItf {
        let (client, connection) = tokio_postgres::connect("postgresql://server:server@localhost:5432/thermostat", NoTls).await.unwrap();

        let mut dbitf = DbItf {
            client,
            prepared_queries: HashMap::new()
        };
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("DB connection error: {}", e);
            }
        });

        heater_timeslot::prepare_all(&mut dbitf).await;
        temperature::prepare_all(&mut dbitf).await;
        setting::prepare_all(&mut dbitf).await;

        return dbitf;
    }

    async fn prepare_from_file(&mut self, query_name: &str) {
        let filename = format!("{}{}.sql", QUERY_PREFIX, query_name);
        let error = format!("Can't open query file {}", filename);
        let contents = fs::read_to_string(filename).expect(error.as_str(), );
        self.prepared_queries.insert(query_name.to_string(), self.client.prepare(&contents).await.unwrap());
    }

    pub async fn transaction(&mut self) -> DbTransaction<'_> {
        DbTransaction {
            prepared_queries: &self.prepared_queries,
            transaction: self.client.transaction().await.unwrap()
        }
    }
}

impl DbTransaction<'_> {
    pub async fn query(
        &self,
        name: &str,
        params: &[&(dyn ToSql + Sync)]
    ) -> Result<Vec<Row>, Error> {
        return self.transaction.query(&self.prepared_queries[name], params).await;
    }

    pub async fn commit(self) {
        self.transaction.commit().await.unwrap();
    }
}