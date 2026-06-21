# AWS Services → Open Source Upstream Mappings

> Research compiled for TotalStack (LocalStack fork). 
> Each entry maps an AWS service to its upstream OSS project with canonical API documentation URLs.

---

## Clear Wrappers (AWS runs/offers the OSS directly)

| AWS Service | Upstream OSS | Upstream API Docs URL | Notes |
|---|---|---|---|
| **ElastiCache** (Redis OSS) | Redis OSS | https://redis.io/docs/latest/ | Supports Redis OSS commands. Also supports Valkey (see below). |
| **ElastiCache** (Valkey) | Valkey | https://valkey.io/docs/ | Valkey is a Linux Foundation fork of Redis 7.2 after Redis license change (2024). API-compatible with Redis. |
| **ElastiCache** (Memcached) | Memcached | https://docs.memcached.org/ | Wire protocol spec: https://github.com/memcached/memcached/blob/master/doc/protocol.txt |
| **MemoryDB** (Redis OSS) | Redis OSS | https://redis.io/docs/latest/ | Durable in-memory DB. Same Redis API. Also supports Valkey. |
| **MemoryDB** (Valkey) | Valkey | https://valkey.io/docs/ | Same as above, Valkey variant. |
| **MSK** (Managed Streaming for Apache Kafka) | Apache Kafka | https://kafka.apache.org/documentation/ | Runs open-source Kafka. Data-plane ops use native Kafka APIs. |
| **Amazon MQ** (ActiveMQ) | Apache ActiveMQ | https://activemq.apache.org/components/classic/documentation/ | Managed ActiveMQ broker. Supports OpenWire, STOMP, AMQP, MQTT. |
| **Amazon MQ** (RabbitMQ) | RabbitMQ | https://www.rabbitmq.com/docs | Managed RabbitMQ broker. AMQP 0-9-1 protocol. Management HTTP API: https://www.rabbitmq.com/docs/http-api-reference |
| **OpenSearch Service** | OpenSearch | https://docs.opensearch.org/latest/api-reference/ | Forked from Elasticsearch 7.10 after Elastic license change. REST API compatible with ES. |
| **Keyspaces** (for Apache Cassandra) | Apache Cassandra | https://cassandra.apache.org/doc/latest/ | CQL 3.11 API-compatible. Data-plane uses standard Cassandra drivers. |
| **Neptune** (Gremlin) | Apache TinkerPop Gremlin | https://tinkerpop.apache.org/docs/current/reference/ | Property graph traversal language. TinkerPop 3.x compatible. |
| **Neptune** (openCypher) | openCypher | https://opencypher.org/ | Originally developed by Neo4j, open-sourced under Apache 2.0. Property graph queries. |
| **Neptune** (SPARQL) | W3C SPARQL | https://www.w3.org/TR/sparql11-query/ | RDF graph query language. W3C standard. |
| **Managed Blockchain** (Hyperledger Fabric) | Hyperledger Fabric | https://hyperledger-fabric.readthedocs.io/ | Private/permissioned blockchain framework. |
| **Managed Blockchain** (Ethereum) | Ethereum | https://ethereum.org/developers/docs/apis/json-rpc/ | Public blockchain. JSON-RPC API. Spec: https://ethereum.github.io/execution-apis/ |
| **Managed Blockchain** (Bitcoin) | Bitcoin Core | https://developer.bitcoin.org/reference/rpc/ | Public blockchain access. Bitcoin JSON-RPC API (AMB Access Bitcoin). |
| **EMR** (Hadoop) | Apache Hadoop | https://hadoop.apache.org/docs/ | HDFS, MapReduce, YARN. |
| **EMR** (Spark) | Apache Spark | https://spark.apache.org/docs/latest/ | EMR has optimized Spark runtime, API-compatible with OSS Spark. |
| **EMR** (Hive) | Apache Hive | https://hive.apache.org/ | Data warehouse on Hadoop. |
| **EMR** (Presto/Trino) | Trino (formerly PrestoSQL) | https://trino.io/docs/current/ | Distributed SQL query engine. |
| **EMR** (Flink) | Apache Flink | https://nightlies.apache.org/flink/flink-docs-stable/ | Stream processing on EMR. |
| **EMR Serverless** (Spark, Hive) | Apache Spark / Apache Hive | https://spark.apache.org/docs/latest/ | Serverless deployment option for EMR — same OSS engines. |
| **MWAA** (Managed Workflows for Apache Airflow) | Apache Airflow | https://airflow.apache.org/docs/ | Managed Airflow. REST API: https://airflow.apache.org/docs/apache-airflow/stable/stable-rest-api-ref.html |
| **RDS** (MySQL) | MySQL Community Edition | https://dev.mysql.com/doc/ | Wire-protocol compatible. |
| **RDS** (PostgreSQL) | PostgreSQL | https://www.postgresql.org/docs/ | Wire-protocol compatible. |
| **RDS** (MariaDB) | MariaDB Server | https://mariadb.com/docs/ | Wire-protocol compatible. Fork of MySQL. |
| **RDS** (Oracle) | Oracle Database | (proprietary, but wire-protocol standard) | Commercial. |
| **RDS** (SQL Server) | Microsoft SQL Server | (proprietary, but wire-protocol standard) | Commercial. |
| **RDS** (Db2) | IBM Db2 | (proprietary, but wire-protocol standard) | Commercial. |
| **Timestream for InfluxDB** | InfluxDB | https://docs.influxdata.com/influxdb/ | Managed InfluxDB 2.x and 3.x. Uses InfluxDB open-source APIs, Telegraf, Flux query language. |
| **Managed Service for Prometheus (AMP)** | Prometheus | https://prometheus.io/docs/prometheus/latest/querying/api/ | Prometheus-compatible remote-write/query APIs. Backed by Cortex internally. |
| **Managed Grafana** | Grafana | https://grafana.com/docs/grafana/latest/developer-resources/api-reference/ | Managed Grafana workspace. Full Grafana HTTP API. |
| **EKS** (Elastic Kubernetes Service) | Kubernetes | https://kubernetes.io/docs/reference/ | Certified Kubernetes conformant. Runs upstream K8s. |
| **Managed Service for Apache Flink** | Apache Flink | https://nightlies.apache.org/flink/flink-docs-stable/ | Formerly Kinesis Data Analytics for Apache Flink. |
| **AWS Glue** (Spark) | Apache Spark | https://spark.apache.org/docs/latest/ | Managed ETL on optimized Spark runtime. |
| **AWS Glue** (Ray) | Ray | https://docs.ray.io/ | Python distributed computing framework. |

---

## Borderline / API-Compatible (proprietary engine, OSS-compatible API)

| AWS Service | Compatible With | Upstream API Docs URL | Notes |
|---|---|---|---|
| **DocumentDB** (with MongoDB compatibility) | MongoDB | https://www.mongodb.com/docs/ | **Proprietary engine.** Implements MongoDB 3.6/4.0/5.0/8.0 wire protocol on AWS's own storage layer. NOT a MongoDB wrapper. Drivers/tools work unchanged. Note: Linux Foundation "DocumentDB" OSS project is a *different* thing (PostgreSQL-based, MongoDB API-compatible) — AWS joined it in 2025. |
| **Aurora** (MySQL-compatible) | MySQL | https://dev.mysql.com/doc/ | **Proprietary engine.** Wire-protocol compatible with MySQL. AWS's own distributed storage engine. |
| **Aurora** (PostgreSQL-compatible) | PostgreSQL | https://www.postgresql.org/docs/ | **Proprietary engine.** Wire-protocol compatible with PostgreSQL. |
| **DAX** (DynamoDB Accelerator) | DynamoDB API (NOT Memcached) | https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/ | **Proprietary wire protocol.** API-compatible with DynamoDB (read operations). NOT Memcached-compatible. It's a read-through/write-through cache specifically for DynamoDB tables. The user's assumption of "Memcached-compatible" is incorrect — that's ElastiCache. |

---

## Original Timestream Caveat

| AWS Service | Compatible With | Upstream API Docs URL | Notes |
|---|---|---|---|
| **Timestream for LiveAnalytics** (original) | N/A (proprietary) | N/A | **Not InfluxDB-based.** This is AWS's own proprietary time-series engine with its own SQL-like query language. Only "Timestream for InfluxDB" wraps InfluxDB. The two are separate services/engines. |

---

## Key License/History Notes

- **Valkey** was forked from Redis 7.2 in March 2024 after Redis Ltd. changed from BSD-3-Clause to RSALv2/SSPLv1 dual license. Linux Foundation project.
- **OpenSearch** was forked from Elasticsearch 7.10 in 2021 after Elastic changed from Apache 2.0 to SSPL/Elastic License. AWS-led, now Linux Foundation project.
- **openCypher** was developed by Neo4j then open-sourced under Apache 2.0 in 2015. Used by Neptune.
- **Cortex** is the backend for Amazon Managed Prometheus (AMP) — it's a CNCF incubating project providing horizontally-scalable Prometheus.
