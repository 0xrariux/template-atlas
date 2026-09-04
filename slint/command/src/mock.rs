use crate::{
    ActivityRecord, AlertRecord, HealthRecord, IntegrationRecord, LogRecord, MetricRecord,
    RegionRecord, ServiceRecord, TeamRecord,
};

pub fn metrics() -> Vec<MetricRecord> {
    [
        (
            "Active Users",
            "12,842",
            "+8.2%",
            "Compared with previous period",
            0,
        ),
        (
            "API Requests",
            "2.48M",
            "+12.4%",
            "Across 18 production services",
            1,
        ),
        ("Error Rate", "0.18%", "−0.04%", "Below 0.25% target", 2),
        ("Uptime", "99.982%", "+0.002%", "30-day rolling average", 3),
    ]
    .into_iter()
    .map(|(label, value, trend, context, style)| MetricRecord {
        label: label.into(),
        value: value.into(),
        trend: trend.into(),
        context: context.into(),
        style,
    })
    .collect()
}

pub fn services() -> Vec<ServiceRecord> {
    [
        (
            "API Gateway",
            "Healthy",
            "Global",
            "842.4K",
            "42 ms",
            "0.08%",
            "v3.18.2",
            0,
        ),
        (
            "Auth Service",
            "Healthy",
            "eu-west-1",
            "492.8K",
            "31 ms",
            "0.12%",
            "v2.9.4",
            0,
        ),
        (
            "Indexer",
            "Degraded",
            "eu-central-1",
            "388.2K",
            "126 ms",
            "0.41%",
            "v4.2.0",
            1,
        ),
        (
            "Payments",
            "Healthy",
            "us-east-1",
            "286.1K",
            "64 ms",
            "0.09%",
            "v1.14.8",
            0,
        ),
        (
            "Notifications",
            "Healthy",
            "Global",
            "249.7K",
            "38 ms",
            "0.04%",
            "v2.6.1",
            0,
        ),
        (
            "Analytics Worker",
            "Maintenance",
            "eu-west-2",
            "219.3K",
            "82 ms",
            "0.17%",
            "v5.0.3",
            2,
        ),
    ]
    .into_iter()
    .map(
        |(name, status, region, requests, latency, errors, version, tone)| ServiceRecord {
            initials: name
                .chars()
                .filter(|c| c.is_ascii_uppercase())
                .take(2)
                .collect::<String>()
                .into(),
            name: name.into(),
            status: status.into(),
            region: region.into(),
            requests: requests.into(),
            latency: latency.into(),
            errors: errors.into(),
            version: version.into(),
            tone,
        },
    )
    .collect()
}

pub fn health_services() -> Vec<HealthRecord> {
    [
        ("API Gateway", "42 ms", "Healthy", 0),
        ("Authentication", "31 ms", "Healthy", 0),
        ("PostgreSQL", "18 ms", "Healthy", 0),
        ("Indexer", "126 ms", "Degraded", 1),
        ("Workers", "54 ms", "Healthy", 0),
        ("Object Storage", "69 ms", "Healthy", 0),
    ]
    .into_iter()
    .map(|(name, latency, status, tone)| HealthRecord {
        name: name.into(),
        latency: latency.into(),
        status: status.into(),
        tone,
    })
    .collect()
}

pub fn activities() -> Vec<ActivityRecord> {
    [
        (
            "Production deployment completed",
            "api-gateway v3.18.2 promoted to 8 instances",
            "Platform Operator",
            "Deployment",
            "14:38:12",
            0,
        ),
        (
            "Workspace member invited",
            "analyst@example.invalid added as Analyst",
            "Demo Administrator",
            "Access",
            "14:22:48",
            1,
        ),
        (
            "Production API key rotated",
            "billing-worker credentials updated",
            "System",
            "Security",
            "13:59:03",
            2,
        ),
        (
            "Alert policy modified",
            "Indexer latency threshold changed 150 → 120 ms",
            "Reliability Operator",
            "Config",
            "13:41:26",
            3,
        ),
        (
            "Database snapshot completed",
            "postgres-primary · 84.2 GB · encrypted",
            "System",
            "Backup",
            "12:18:52",
            4,
        ),
        (
            "Worker pool scaled",
            "analytics-worker increased 4 → 8 instances",
            "Platform Operator",
            "Scale",
            "11:46:10",
            5,
        ),
    ]
    .into_iter()
    .map(
        |(title, detail, actor, category, time, icon)| ActivityRecord {
            title: title.into(),
            detail: detail.into(),
            actor: actor.into(),
            category: category.into(),
            time: time.into(),
            icon,
        },
    )
    .collect()
}

pub fn alerts() -> Vec<AlertRecord> {
    [
        (
            "Critical",
            "High latency detected",
            "Indexer",
            "eu-central-1",
            "18m 42s",
            "Unassigned",
            "Investigating",
            0,
        ),
        (
            "Warning",
            "Event processing lag",
            "Indexer",
            "region-demo-1",
            "31m 08s",
            "Platform Operator",
            "Acknowledged",
            1,
        ),
        (
            "Warning",
            "Certificate expires soon",
            "API Gateway",
            "Global",
            "14 days",
            "Platform",
            "Open",
            1,
        ),
        (
            "Information",
            "Backup retention threshold",
            "PostgreSQL",
            "eu-west-1",
            "2h 14m",
            "System",
            "Open",
            2,
        ),
    ]
    .into_iter()
    .enumerate()
    .map(
        |(index, (severity, title, service, region, duration, owner, status, tone))| AlertRecord {
            id: index as i32 + 1,
            severity: severity.into(),
            title: title.into(),
            service: service.into(),
            region: region.into(),
            duration: duration.into(),
            owner: owner.into(),
            status: status.into(),
            tone,
        },
    )
    .collect()
}

pub fn logs() -> Vec<LogRecord> {
    [
        (
            "14:42:18.492",
            "api-gateway",
            "INFO",
            "Request completed",
            "method=POST path=/v1/events status=202 duration=42ms",
            0,
        ),
        (
            "14:42:18.128",
            "indexer",
            "WARN",
            "Consumer lag above threshold",
            "partition=04 lag=12420 threshold=10000",
            1,
        ),
        (
            "14:42:17.842",
            "auth-service",
            "INFO",
            "Session token issued",
            "subject=usr_28a91 ttl=3600 scope=workspace",
            0,
        ),
        (
            "14:42:16.990",
            "postgres",
            "DEBUG",
            "Query completed",
            "duration=18ms rows=42 pool=primary",
            3,
        ),
        (
            "14:42:15.774",
            "indexer",
            "ERROR",
            "Batch processing timeout",
            "batch=evt_9218 elapsed=5002ms retry=2",
            2,
        ),
        (
            "14:42:14.330",
            "payments",
            "INFO",
            "Payment intent reconciled",
            "intent=pi_9211 amount=24800 currency=EUR",
            0,
        ),
        (
            "14:42:12.108",
            "object-storage",
            "INFO",
            "Multipart upload completed",
            "object=export_2026_09_01.csv size=42MB",
            0,
        ),
    ]
    .into_iter()
    .map(
        |(time, service, level, event, attributes, tone)| LogRecord {
            time: time.into(),
            service: service.into(),
            level: level.into(),
            event: event.into(),
            attributes: attributes.into(),
            tone,
        },
    )
    .collect()
}

pub fn team() -> Vec<TeamRecord> {
    [
        (
            "Demo Administrator",
            "DA",
            "administrator@example.invalid",
            "Administrator",
            "Operations",
            "Active",
            "Now",
            0,
        ),
        (
            "Platform Operator",
            "PO",
            "platform.operator@example.invalid",
            "Operator",
            "Platform",
            "Active",
            "4 min ago",
            0,
        ),
        (
            "Reliability Operator",
            "RO",
            "reliability.operator@example.invalid",
            "Operator",
            "Reliability",
            "Active",
            "18 min ago",
            0,
        ),
        (
            "Demo Analyst",
            "AN",
            "analyst@example.invalid",
            "Analyst",
            "Data",
            "Invited",
            "Pending",
            1,
        ),
        (
            "Finance Viewer",
            "FV",
            "finance.viewer@example.invalid",
            "Viewer",
            "Finance",
            "Active",
            "Yesterday",
            0,
        ),
    ]
    .into_iter()
    .map(
        |(name, initials, email, role, team, status, last, tone)| TeamRecord {
            name: name.into(),
            initials: initials.into(),
            email: email.into(),
            role: role.into(),
            team: team.into(),
            status: status.into(),
            last: last.into(),
            tone,
        },
    )
    .collect()
}

pub fn integrations() -> Vec<IntegrationRecord> {
    [
        (
            "GitHub",
            "SOURCE CONTROL",
            "Link deployments and commits to service activity.",
            "Connected",
            0,
            0,
        ),
        (
            "Slack",
            "NOTIFICATIONS",
            "Route alerts and reports to team channels.",
            "Connected",
            0,
            1,
        ),
        (
            "Datadog",
            "OBSERVABILITY",
            "Synchronize monitors, incidents, and telemetry.",
            "Connected",
            0,
            2,
        ),
        (
            "PagerDuty",
            "INCIDENT RESPONSE",
            "Escalate critical alerts to on-call schedules.",
            "Available",
            1,
            3,
        ),
        (
            "AWS",
            "INFRASTRUCTURE",
            "Import regions, services, and cloud health data.",
            "Available",
            1,
            4,
        ),
        (
            "Webhook",
            "DEVELOPER TOOLS",
            "Push signed Atlas events to a custom endpoint.",
            "Configure",
            2,
            5,
        ),
    ]
    .into_iter()
    .map(
        |(name, category, description, status, tone, icon)| IntegrationRecord {
            name: name.into(),
            category: category.into(),
            description: description.into(),
            status: status.into(),
            tone,
            icon,
        },
    )
    .collect()
}

pub fn regions() -> Vec<RegionRecord> {
    [
        ("Europe West", "eu-west-1", "942.8K", "38 ms", 0.91),
        ("US East", "us-east-1", "718.4K", "54 ms", 0.70),
        ("Europe Central", "eu-central-1", "495.2K", "31 ms", 0.48),
        ("Asia Pacific", "ap-southeast-1", "323.6K", "91 ms", 0.31),
    ]
    .into_iter()
    .map(|(name, code, value, latency, share)| RegionRecord {
        name: name.into(),
        code: code.into(),
        value: value.into(),
        latency: latency.into(),
        share,
    })
    .collect()
}
