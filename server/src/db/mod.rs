use tokio_postgres::{Connection, Client, Error, Socket, Statement};
use tokio_postgres::tls::{NoTls, NoTlsStream};
use tokio::net::TcpStream;
use tokio_postgres::types::ToSql;
use tokio_postgres::row::Row;
use std::collections::HashMap;
use std::fs;

static QUERY_PREFIX: &str = "sql/query/";

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
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("DB connection error: {}", e);
            }
        });

        dbitf.prepare_from_file("heater_timeslot/insert").await;

        return dbitf;
    }

    async fn prepare_from_file(&mut self, query_name: &str) {
        let filename = format!("{}{}.sql", QUERY_PREFIX, query_name);
        let contents = fs::read_to_string(filename).expect(format!("Can't open query file {}", filename), );
        self.prepared_queries.insert(query_name.to_string(), self.client.prepare(&contents).await.unwrap());
    }

    pub async fn query(
        &self,
        name: &str,
        params: &[&(dyn ToSql + Sync)]
    ) -> Result<Vec<Row>, Error> {
        return self.client.query(&self.prepared_queries[name], params).await;
    }
}

