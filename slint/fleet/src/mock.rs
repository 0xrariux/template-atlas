use crate::{DeploymentRecord, DeviceRecord, IncidentRecord, LogRecord, WorkflowRecord};

pub fn devices() -> Vec<DeviceRecord> {
    [
        (
            "RPI-DEMO-01",
            "Raspberry Pi 5",
            "SITE-ALPHA / RACK A",
            "ONLINE",
            0,
            34,
            "3.8 / 8 GB",
            48,
            48,
            62,
            4,
            "192.0.2.42",
            "ARM64",
        ),
        (
            "RPI-DEMO-02",
            "Raspberry Pi 5",
            "SITE-BETA / LAB 02",
            "WARNING",
            1,
            82,
            "6.2 / 8 GB",
            78,
            72,
            71,
            5,
            "192.0.2.16",
            "ARM64",
        ),
        (
            "NODE-DEMO-01",
            "Rack Server Model A",
            "SITE-ALPHA / RACK C",
            "ONLINE",
            0,
            68,
            "42 / 128 GB",
            33,
            41,
            46,
            12,
            "198.51.100.12",
            "X86_64",
        ),
        (
            "CLOUD-DEMO-01",
            "Cloud VM Medium",
            "REGION-DEMO-01",
            "ONLINE",
            0,
            22,
            "14 / 64 GB",
            22,
            38,
            58,
            8,
            "198.51.100.118",
            "X86_64",
        ),
        (
            "WORKER-DEMO-02",
            "Rack Server Model B",
            "SITE-ALPHA / RACK B",
            "MAINTENANCE",
            2,
            0,
            "0 / 64 GB",
            0,
            29,
            77,
            0,
            "203.0.113.44",
            "X86_64",
        ),
        (
            "EDGE-DEMO-03",
            "Edge Compute Module",
            "SITE-GAMMA / EDGE",
            "OFFLINE",
            3,
            0,
            "— / 8 GB",
            0,
            0,
            38,
            0,
            "203.0.113.8",
            "ARM64",
        ),
    ]
    .into_iter()
    .map(
        |(
            id,
            model,
            location,
            state,
            tone,
            cpu,
            memory,
            memory_percent,
            temp,
            disk,
            services,
            ip,
            kind,
        )| DeviceRecord {
            id: id.into(),
            model: model.into(),
            location: location.into(),
            state: state.into(),
            tone,
            cpu,
            memory: memory.into(),
            memory_percent,
            temp,
            disk,
            services,
            ip: ip.into(),
            kind: kind.into(),
        },
    )
    .collect()
}

pub fn deployments() -> Vec<DeploymentRecord> {
    [
        (
            "RPI-DEMO-01",
            "SITE-ALPHA / RACK A",
            "COMPLETED",
            0,
            100,
            "14:03",
            "1m 42s",
        ),
        (
            "NODE-DEMO-01",
            "SITE-ALPHA / RACK C",
            "COMPLETED",
            0,
            100,
            "14:04",
            "2m 08s",
        ),
        (
            "RPI-DEMO-02",
            "SITE-BETA / LAB 02",
            "DEPLOYING",
            1,
            68,
            "14:31",
            "11m 18s",
        ),
        (
            "CLOUD-DEMO-01",
            "REGION-DEMO-01",
            "DEPLOYING",
            1,
            42,
            "14:36",
            "6m 04s",
        ),
        (
            "WORKER-DEMO-02",
            "SITE-ALPHA / RACK B",
            "QUEUED",
            2,
            0,
            "—",
            "—",
        ),
        (
            "EDGE-DEMO-03",
            "SITE-GAMMA / EDGE",
            "FAILED",
            3,
            24,
            "14:18",
            "4m 51s",
        ),
    ]
    .into_iter()
    .map(
        |(device, location, state, tone, progress, started, duration)| DeploymentRecord {
            device: device.into(),
            location: location.into(),
            state: state.into(),
            tone,
            progress,
            started: started.into(),
            duration: duration.into(),
        },
    )
    .collect()
}

pub fn logs() -> Vec<LogRecord> {
    [
        (
            "14:42:18.048",
            "RPI-DEMO-01",
            "atlas-agent",
            "INFO",
            0,
            "Telemetry packet transmitted · seq=482118",
        ),
        (
            "14:42:17.912",
            "NODE-DEMO-01",
            "indexer",
            "WARN",
            1,
            "Queue depth above threshold · depth=12,481",
        ),
        (
            "14:42:16.734",
            "EDGE-DEMO-03",
            "heartbeat",
            "ERROR",
            2,
            "Device heartbeat timeout after 30m",
        ),
        (
            "14:42:14.380",
            "RPI-DEMO-02",
            "sensors",
            "WARN",
            1,
            "Temperature threshold exceeded · sensor=cpu · 72°C",
        ),
        (
            "14:42:11.105",
            "CLOUD-DEMO-01",
            "database",
            "INFO",
            0,
            "Checkpoint complete · buffers=1928",
        ),
    ]
    .into_iter()
    .map(|(time, device, service, level, tone, message)| LogRecord {
        time: time.into(),
        device: device.into(),
        service: service.into(),
        level: level.into(),
        tone,
        message: message.into(),
    })
    .collect()
}

pub fn incidents() -> Vec<IncidentRecord> {
    [
        (
            "EDGE-DEMO-03",
            "CRITICAL",
            "Device heartbeat lost",
            "No heartbeat received for 32 minutes.",
            "32M",
            "OPEN",
            "UNASSIGNED",
            2,
        ),
        (
            "NODE-DEMO-01",
            "WARNING",
            "Sustained high CPU",
            "CPU above 92% for 8 minutes.",
            "8M",
            "ACKNOWLEDGED",
            "DEMO ADMIN",
            1,
        ),
        (
            "RPI-DEMO-02",
            "WARNING",
            "Thermal threshold exceeded",
            "CPU temperature reached 72°C.",
            "12M",
            "OPEN",
            "OPS DEMO",
            1,
        ),
        (
            "WORKER-DEMO-02",
            "INFO",
            "Maintenance window active",
            "Scheduled maintenance ends at 15:30.",
            "42M",
            "PLANNED",
            "SYSTEM",
            0,
        ),
    ]
    .into_iter()
    .map(
        |(device, severity, title, detail, time, status, owner, tone)| IncidentRecord {
            device: device.into(),
            severity: severity.into(),
            title: title.into(),
            detail: detail.into(),
            time: time.into(),
            status: status.into(),
            owner: owner.into(),
            tone,
        },
    )
    .collect()
}

pub fn workflows() -> Vec<WorkflowRecord> {
    [
        (
            "Restart degraded service",
            "Service status = DEGRADED for 5m",
            3,
            28,
            "96%",
            "ACTIVE",
            0,
        ),
        (
            "Thermal protection",
            "Temperature > 75°C",
            4,
            6,
            "100%",
            "ACTIVE",
            0,
        ),
        (
            "Offline device recovery",
            "Heartbeat missing for 10m",
            5,
            12,
            "83%",
            "ACTIVE",
            1,
        ),
        (
            "Nightly configuration backup",
            "Schedule · 02:00 CET",
            2,
            18,
            "100%",
            "ACTIVE",
            0,
        ),
        (
            "Rotate sample credentials",
            "Manual trigger",
            3,
            0,
            "—",
            "DRAFT",
            2,
        ),
    ]
    .into_iter()
    .map(
        |(name, trigger, actions, runs, success, state, tone)| WorkflowRecord {
            name: name.into(),
            trigger: trigger.into(),
            actions,
            runs,
            success: success.into(),
            state: state.into(),
            tone,
        },
    )
    .collect()
}
