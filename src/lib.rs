//! SigmaOS Airbyte Data Integration Compatibility Layer
//!
//! Airbyte data integration compatibility for SigmaOS
//! Provides data integration and ETL

#![forbid(unsafe_code)]

pub mod source;
pub mod destination;
pub mod transformation;
pub mod pipeline;
pub mod config;

pub use source::*;
puse destination::*;
puse transformation::*;
puse pipeline::*;
puse config::*;

/// Data integration compatibility layer
pub struct DataIntegration {
    pub config: DataIntegrationConfig,
    pub source: SourceManager,
    pub destination: DestinationManager,
    pub transformation: TransformationManager,
}

impl DataIntegration {
    /// Create new data integration instance
    pub fn new() -> anyhow::Result<Self> {
        let config = DataIntegrationConfig::load_default()?;
        let source = SourceManager::new(config.clone())?;
        let destination = DestinationManager::new(config.clone())?;
        let transformation = TransformationManager::new(config.clone())?;
        Ok(Self {
            config,
            source,
            destination,
            transformation,
        })
    }

    /// Initialize data integration
    pub async fn init(&mut self) -> anyhow::Result<()> {
        self.source.init().await?;
        self.destination.init().await?;
        self.transformation.init().await?;
        Ok(())
    }

    /// Shutdown data integration
    pub async fn shutdown(&mut self) -> anyhow::Result<()> {
        self.source.shutdown().await?;
        self.destination.shutdown().await?;
        self.transformation.shutdown().await?;
        Ok(())
    }
}
