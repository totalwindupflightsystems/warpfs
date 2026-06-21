# AWS Services Priority Matrix for TotalStack

> Research compiled June 2026. Based on LocalStack usage data, Datadog State of Serverless reports,
> AWS service deprecation announcements (July 2024, May 2025, October 2025), and community demand analysis.
> TotalStack currently has **19 services completed** (all in Tier 1). Target is ~370 AWS services total.

---

## TIER 1 — MUST-HAVE (19 of 19 done ✅)

These are the services **every** AWS developer expects in a local emulator. Per Datadog's State of Serverless,
the top 10 AWS services account for **89% of all API calls** in production workloads.

| # | Service | Status | Why |
|---|---------|--------|-----|
| 1 | **S3** | ✅ Done | #1 most-used AWS service; object storage backbone |
| 2 | **Lambda** | ✅ Done | 65%+ of AWS customers use Lambda; serverless core |
| 3 | **DynamoDB** | ✅ Done | Primary serverless database |
| 4 | **SQS** | ✅ Done | #1 message queue for Lambda; dead-letter queues |
| 5 | **SNS** | ✅ Done | Pub/sub messaging; triggers Lambda + SQS |
| 6 | **IAM** | ✅ Done | Required for ANY multi-service interaction |
| 7 | **STS** | ✅ Done | Required for IAM role assumption, cross-account |
| 8 | **CloudWatch** | ✅ Done | Every AWS service emits metrics here |
| 9 | **CloudWatch Logs** | ✅ Done | Lambda logging, all service logs |
| 10 | **API Gateway** | ✅ Done | REST/HTTP API frontend for Lambda + services |
| 11 | **EC2** | ✅ Done | 68% of survey respondents say most critical service |
| 12 | **ECS** | ✅ Done | Primary AWS container orchestrator |
| 13 | **Step Functions** | ✅ Done | Serverless workflow orchestration |
| 14 | **EventBridge** | ✅ Done | Event bus; replaces CloudWatch Events |
| 15 | **Route 53** | ✅ Done | DNS; required for any domain-based testing |
| 16 | **KMS** | ✅ Done | Encryption keys; used by S3, Lambda, DynamoDB, SQS, SNS |
| 17 | **Secrets Manager** | ✅ Done | Secrets storage; heavily used in CI/CD pipelines |
| 18 | **CloudFormation** | ✅ Done | IaC; Terraform/CDK/SAM all depend on it |
| 19 | **ELB** (ALB/NLB) | ✅ Done | Load balancing for EC2, ECS, Lambda |

**All 19 are complete.** These are the "table stakes" — without them, TotalStack is non-viable.

---

## TIER 2 — HIGH VALUE (build next, ~35 services)

Commonly used in CI/CD, local dev, and integration testing. High community demand.

### Compute & Containers
| Service | Priority | Notes |
|---------|----------|-------|
| EKS | Very High | K8s is everywhere |
| ECR | Very High | Required to push/pull images with ECS/EKS |
| Auto Scaling | High | EC2 auto-scaling groups |
| Elastic Beanstalk | Medium | PaaS; declining but still widely used |
| App Runner | Medium | Simplified container deployments |

### Databases
| Service | Priority | Notes |
|---------|----------|-------|
| RDS | Very High | Most-used AWS database; PostgreSQL/MySQL/MariaDB |
| ElastiCache | Very High | Redis/Memcached caching layer |
| DynamoDB Streams | High | CDC for DynamoDB; triggers Lambdas |
| Redshift | Medium | Data warehousing |
| DocumentDB | Medium | MongoDB compatibility |
| Neptune | Low-Medium | Graph database |
| MemoryDB | Low | Redis-compatible in-memory DB |

### Networking & CDN
| Service | Priority | Notes |
|---------|----------|-------|
| CloudFront | Very High | CDN; S3+CloudFront is most common static hosting |
| ACM | High | TLS certificates |
| Route 53 Resolver | Medium | DNS resolution rules |
| Private CA | Low | Private certificate authority |

### Security & Identity
| Service | Priority | Notes |
|---------|----------|-------|
| Cognito | Very High | Auth for web/mobile apps; extremely common |
| WAF | Medium | WAF rules for CloudFront/ALB |
| Shield | Low | DDoS protection |
| GuardDuty | Low | Threat detection |

### Messaging & Streaming
| Service | Priority | Notes |
|---------|----------|-------|
| Kinesis Data Streams | High | Real-time data streaming |
| MSK (Managed Kafka) | Medium | Managed Kafka |
| EventBridge Pipes | Medium | Point-to-point integrations |
| EventBridge Scheduler | Medium | Cron-like scheduling |
| Kinesis Firehose | Medium | Data delivery streams |

### Developer Tools (CI/CD)
| Service | Priority | Notes |
|---------|----------|-------|
| CodeBuild | High | Managed builds; CI/CD core |
| CodePipeline | High | CI/CD pipeline orchestration |
| CodeDeploy | Medium | Deployment automation |
| CodeArtifact | Medium | Package registry (npm, pip, maven) |

### Management & Monitoring
| Service | Priority | Notes |
|---------|----------|-------|
| CloudTrail | Very High | API audit logs; critical for IAM testing |
| Config | High | Resource compliance/inventory |
| SSM (Systems Manager) | High | Parameter Store heavily used |
| X-Ray | Medium | Distributed tracing |
| Application Auto Scaling | Medium | Dynamic scaling |

### Storage & Backup
| Service | Priority | Notes |
|---------|----------|-------|
| EFS | High | NFS for Lambda + ECS + EC2 |
| Backup | Medium | Centralized backup management |

### Analytics
| Service | Priority | Notes |
|---------|----------|-------|
| Athena | High | SQL on S3 |
| Glue | High | ETL service; data catalog |
| EMR | Medium | Big data; Hadoop/Spark |

### Other High-Value
| Service | Priority | Notes |
|---------|----------|-------|
| Batch | High | Batch computing; HPC workloads |
| SES | Medium | Email sending |
| OpenSearch | Medium | Search/analytics engine |
| Organizations | Medium | Multi-account management |
| S3 Tables | Low-Medium | Iceberg-compatible table format |
| Bedrock | Medium | AI/ML foundation models |

---

## TIER 3 — NICE TO HAVE (build later, ~30-40 services)

Used by some teams. Good candidates for community contributions.

| Service | Notes |
|---------|-------|
| AppSync | GraphQL API; growing |
| AppConfig | Feature flags; simple |
| App Mesh | Service mesh; niche but growing |
| Amplify | Frontend hosting |
| SageMaker | ML; very complex, rarely tested locally |
| Textract / Transcribe / Rekognition / Comprehend / Polly / Translate / Lex | AI services; rarely tested locally |
| IoT Core / IoT Data | IoT device management |
| Greengrass V2 | Edge runtime |
| FIS | Chaos engineering |
| DMS | DB migration (fleet advisor deprecated, core DMS ok) |
| Transfer Family | SFTP/FTPS/FTP |
| SSO Admin / Identity Store | Enterprise SSO |
| RAM | Cross-account resource sharing |
| Resource Groups Tagging API | Simple tag passthrough |
| MWAA | Managed Airflow; popular |
| FSx | Managed file systems; niche |
| Global Accelerator | Network optimization |
| Site-to-Site VPN / Client VPN | VPN connections |
| Transit Gateway | Network hub |
| Network Firewall | Network filtering |
| API Gateway WebSocket | WebSocket APIs |
| DataSync | Data transfer |
| Storage Gateway | On-prem bridge |
| Verified Permissions | Cedar policy engine |

---

## TIER 4 — SKIP / NEVER BUILD (~110 services)

### A. DEPRECATED / END-OF-LIFE (55+ services)

Services confirmed by AWS as no longer accepting new customers or fully shut down.

**July 2024 — No new customers:** SimpleDB, S3 Select, CloudSearch, Cloud9, Forecast, Data Pipeline, CodeCommit, CodeStar

**May 2025 — Maintenance mode:** IoT Events, SimSpace Weaver, Panorama, Connect Voice ID, Inspector Classic, AWS IQ

**October 2025 — Maintenance mode (17 more):** Cloud Directory, CodeCatalyst, S3 Object Lambda, Snowball Edge, Timestream LiveAnalytics, DMS Fleet Advisor, IoT Greengrass V1, Systems Manager Change Manager, Systems Manager Incident Manager, and ~8 more lesser-known services

**Full shutdown (completed):** QLDB (Jul 2025), WorkDocs (Apr 2025), WorkLink (Nov 2021), OpsWorks all variants (May 2024), DeepComposer (Sep 2025), DeepLens (Jan 2024), Elastic Transcoder (Nov 2025), MediaStore (Nov 2025), Lookout for Metrics (Oct 2025), Lookout for Vision (Oct 2025), Nimble Studio (Jun 2024), Sumerian (Mar 2023), Lumberyard (May 2021), Mobile Hub (Oct 2021), RDS on VMware (May 2022), Server Migration Service (Apr 2023), IoT Analytics (Dec 2025), IoT 1-Click (Jan 2025), IoT Things Graph (Nov 2022), IoT RoboRunner (Mar 2024), Fleet Hub IoT (Oct 2025), Classic Glacier API (End 2025), WAF Classic (End 2025), CloudWatch Evidently (Oct 2025), Kinesis Data Analytics SQL (Jan 2026), Lake Formation Governed Tables (Dec 2024), BugBust (Aug 2025), Private 5G (May 2025), DataSync Discovery (May 2025), NICE EnginFrame (Sep 2025), Mainframe Modernization App Testing (Oct 2025), Snowball variants (Nov 2024-2025), Snowmobile (Mar 2024)

### B. PHYSICAL HARDWARE REQUIRED (9 services)

Cannot be emulated in software: Snowball/Snowmobile/Snowblade (deprecated anyway), Outposts (physical rack), Ground Station (satellite antennas), Braket (quantum computers), RoboMaker (deprecated), Panorama (edge appliance; deprecated), Private 5G (radio; deprecated), Device Farm (physical mobile devices), Direct Connect (physical fiber)

### C. PURE MANAGEMENT PLANE WRAPPERS (14 services)

Exist only to orchestrate other AWS services. Skip or trivial passthrough.

- **Control Tower** — orchestrates Organizations + Config + CloudTrail + SSO + Service Catalog
- **Service Catalog** — CloudFormation template catalog
- **Resource Groups** — tag-based grouping
- **Support** — AWS internal
- **Account Management** — AWS internal
- **Cost Explorer** — meaningless locally
- **Trusted Advisor** — AWS best-practice checks
- **License Manager** — useless locally
- **Budgets** — meaningless locally
- **Compute Optimizer** — AWS telemetry-based
- **Pricing Calculator** — meaningless
- **Cost & Usage Report** — meaningless
- **Health** — AWS service dashboard
- **AWS IQ** — human marketplace (also deprecated)

### D. ULTRA-NICHE / <0.1% ADOPTION (30+ services)

Services so specialized they're used by almost no one: Managed Blockchain, GameLift, Kendra, Comprehend Medical, HealthLake/HealthImaging/HealthOmics, Connect (call center), Honeycode, AppFlow, B2BI, Supply Chain, Deadline Cloud, FinSpace, Entity Resolution, Clean Rooms, Data Exchange, DataZone, Glue DataBrew, DAX, Keyspaces, Wickr, WorkMail, Chime, Proton, Monitron, IoT SiteWise/TwinMaker/FleetWise, Telco Network Builder, ARC Zonal Shift, Verified Access, Mainframe Modernization

---

## BUILD PRIORITY ROADMAP

```
Phase 1 — Tier 2 Critical (do next, ~8):
  RDS → CloudFront → CloudTrail → Cognito → ElastiCache
  → ECR → Auto Scaling → SSM Parameter Store

Phase 2 — Tier 2 High (do soon, ~12):
  Kinesis → CodeBuild → CodePipeline → Glue → Athena
  → EFS → DynamoDB Streams → WAF → OpenSearch → EKS
  → ACM → EventBridge Pipes/Scheduler

Phase 3 — Tier 2 Medium (do later, ~15):
  Batch → Backup → Redshift → DocumentDB → X-Ray → SES
  → S3 Tables → Organizations → MSK → EMR → AppSync
  → Bedrock → SageMaker → Kinesis Firehose → AppConfig

Phase 4 — Tier 3 Opportunistic (~30):
  Built when needed by teams or contributed by community

Phase 5 — Never Build (~110):
  55+ deprecated, 9 hardware, 14 management wraps, 30+ ultra-niche
```

### Quick Math
```
370   total AWS services (approximate)
− 55  deprecated / end-of-life
−  9  hardware-requiring
− 14  management-plane wrappers
− 30  ultra-niche
─────────────────────────
262   worth considering
− 19  already done (Tier 1)
─────────────────────────
~243  left to potentially build
```

### Build targets
- Tier 2 critical: ~8 services → unblocks most users
- Tier 2 high: ~12 more → covers ~95% of CI/CD and dev workflows
- Tier 2 medium: ~15 more → covers ~98% of use cases
- Tier 3: ~30 more → long tail, build as needed
- Remaining ~178: gray zone — build if a paying customer asks

---

## SOURCES

1. Datadog State of Serverless (2022-2024) — top 10 services = 89% of API calls
2. LocalStack docs (docs.localstack.cloud/aws/services/) — 99 services emulated
3. AWS Service Lifecycle docs (full_shutdown_services.html) — official shutdown list
4. Jeff Barr (AWS, July 2024) — confirmed 7 services closed to new customers
5. AWS October 2025 announcement — 17 maintenance mode, 5 full shutdown
6. Last Week in AWS (Corey Quinn, Oct 2025) — analysis of deprecations
7. Reddit r/aws, r/devops — community discussions
8. AWS Control Tower docs — confirms orchestration-layer architecture
9. Branch8 MiniStack comparison — Datadog top-10 services breakdown
