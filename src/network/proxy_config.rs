//! 代理配置
//!
//! 负责网络代理设置

use std::net::{IpAddr, SocketAddr};
use reqwest::Proxy;
use crate::utils::error::{HamsterError, Result};

#[derive(Debug, Clone)]
pub struct ProxyConfig {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub bypass_hosts: Vec<String>, // 不使用代理的主机列表
}

impl ProxyConfig {
    pub fn new(host: String, port: u16) -> Self {
        Self {
            enabled: false,
            host,
            port,
            username: None,
            password: None,
            bypass_hosts: Vec::new(),
        }
    }

    /// 设置认证信息
    pub fn with_auth(mut self, username: String, password: String) -> Self {
        self.username = Some(username);
        self.password = Some(password);
        self
    }

    /// 添加绕过主机
    pub fn add_bypass_host(mut self, host: String) -> Self {
        self.bypass_hosts.push(host);
        self
    }

    /// 启用代理
    pub fn enable(mut self) -> Self {
        self.enabled = true;
        self
    }

    /// 检查是否应该绕过代理
    pub fn should_bypass(&self, host: &str) -> bool {
        self.bypass_hosts.iter().any(|bypass| {
            host.contains(bypass)
        })
    }

    /// 构建reqwest代理对象
    pub fn build_proxy(&self) -> Result<Proxy> {
        if !self.enabled {
            return Err(HamsterError::ConfigError("代理未启用".to_string()));
        }

        let proxy_url = format!("http://{}:{}", self.host, self.port);
        let mut proxy = Proxy::http(&proxy_url)
            .map_err(|e| HamsterError::NetworkError(format!("代理URL无效: {}", e)))?;

        if let Some(ref username) = self.username {
            if let Some(ref password) = self.password {
                proxy = proxy.basic_auth(username, password);
            }
        }

        Ok(proxy)
    }

    /// 验证代理配置
    pub async fn validate(&self) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        // 尝试连接到代理服务器
        let socket_addr = format!("{}:{}", self.host, self.port)
            .parse::<SocketAddr>()
            .map_err(|e| HamsterError::NetworkError(format!("代理地址无效: {}", e)))?;

        tokio::net::TcpStream::connect(socket_addr).await
            .map_err(|e| HamsterError::NetworkError(format!("无法连接到代理服务器: {}", e)))?;

        Ok(())
    }
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            host: "127.0.0.1".to_string(),
            port: 8080,
            username: None,
            password: None,
            bypass_hosts: vec!["localhost".to_string(), "127.0.0.1".to_string()],
        }
    }
}