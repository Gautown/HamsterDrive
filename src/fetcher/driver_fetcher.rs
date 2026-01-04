use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::Command;
use tokio::fs;
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::process::Child;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadProgress {
    pub file_name: String,
    pub total_size: u64,
    pub downloaded_size: u64,
    pub progress: f32, // 0.0 to 100.0
    pub status: String, // "pending", "downloading", "completed", "failed"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadTask {
    pub id: String,
    pub url: String,
    pub file_path: String,
    pub file_name: String,
    pub expected_size: Option<u64>,
    pub checksum: Option<String>,
}

pub struct DriverFetcher {
    pub aria2_host: String,
    pub aria2_port: u16,
    pub download_dir: String,
    aria2_process: Arc<Mutex<Option<Child>>>,
}

impl DriverFetcher {
    pub fn new(aria2_host: String, aria2_port: u16, download_dir: String) -> Self {
        DriverFetcher {
            aria2_host,
            aria2_port,
            download_dir,
            aria2_process: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn download_driver(&self, task: &DownloadTask) -> Result<DownloadProgress> {
        // 确保下载目录存在
        fs::create_dir_all(&self.download_dir).await?;

        // 使用Aria2进行下载
        let download_path = format!("{}/{}", self.download_dir, task.file_name);
        
        // 构建Aria2命令
        let output = Command::new("aria2c")
            .args(&[
                "--continue=true",
                "--max-connection-per-server=8",
                "--split=8",
                "--max-tries=3",
                "--timeout=60",
                &format!("--dir={}", self.download_dir),
                &format!("--out={}", task.file_name),
                &task.url,
            ])
            .output()?;

        if output.status.success() {
            // 获取下载文件大小
            let metadata = fs::metadata(&download_path).await?;
            let downloaded_size = metadata.len();
            
            Ok(DownloadProgress {
                file_name: task.file_name.clone(),
                total_size: downloaded_size,
                downloaded_size,
                progress: 100.0,
                status: "completed".to_string(),
            })
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Aria2下载失败: {}", error_msg))
        }
    }

    pub async fn download_driver_with_progress(&self, task: &DownloadTask, 
                                              progress_callback: impl Fn(DownloadProgress) -> ()) -> Result<()> {
        // 确保下载目录存在
        fs::create_dir_all(&self.download_dir).await?;

        // 启动Aria2进程进行下载
        let download_path = format!("{}/{}", self.download_dir, task.file_name);
        
        // 首先获取文件大小
        let file_size = self.get_remote_file_size(&task.url).await.unwrap_or(0);
        
        // 通过Aria2下载
        let output = Command::new("aria2c")
            .args(&[
                "--continue=true",
                "--max-connection-per-server=8",
                "--split=8",
                "--max-tries=3",
                "--timeout=60",
                "--enable-rpc=true",
                "--rpc-listen-port=6800",
                &format!("--dir={}", self.download_dir),
                &format!("--out={}", task.file_name),
                &task.url,
            ])
            .output()?;

        if output.status.success() {
            // 下载完成，报告进度
            let metadata = fs::metadata(&download_path).await?;
            let downloaded_size = metadata.len();
            
            let progress = DownloadProgress {
                file_name: task.file_name.clone(),
                total_size: file_size,
                downloaded_size,
                progress: 100.0,
                status: "completed".to_string(),
            };
            
            progress_callback(progress);
            Ok(())
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            let progress = DownloadProgress {
                file_name: task.file_name.clone(),
                total_size: file_size,
                downloaded_size: 0,
                progress: 0.0,
                status: "failed".to_string(),
            };
            
            progress_callback(progress);
            Err(anyhow::anyhow!("Aria2下载失败: {}", error_msg))
        }
    }

    // 通过HTTP获取远程文件大小
    async fn get_remote_file_size(&self, url: &str) -> Result<u64> {
        let response = reqwest::get(url).await?;
        if let Some(content_length) = response.headers().get(reqwest::header::CONTENT_LENGTH) {
            let size_str = content_length.to_str()?;
            let size = size_str.parse::<u64>()?;
            Ok(size)
        } else {
            // 如果无法获取Content-Length，返回0
            Ok(0)
        }
    }

    // 检查下载的文件校验和
    pub async fn verify_checksum(&self, file_path: &str, expected_checksum: &str) -> Result<bool> {
        use sha2::{Sha256, Digest};
        
        let data = fs::read(file_path).await?;
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let result = hasher.finalize();
        let actual_checksum = format!("{:x}", result);
        
        Ok(actual_checksum == expected_checksum)
    }

    // 使用HTTP作为备选下载方式
    pub async fn download_via_http(&self, task: &DownloadTask) -> Result<DownloadProgress> {
        fs::create_dir_all(&self.download_dir).await?;
        
        let download_path = format!("{}/{}", self.download_dir, task.file_name);
        let mut file = tokio::fs::File::create(&download_path).await?;
        
        let response = reqwest::get(&task.url).await?;
        let total_size = response.content_length().unwrap_or(0);
        
        let mut downloaded: u64 = 0;
        let stream = response.bytes_stream();
        
        use tokio::io::AsyncWriteExt;
        use futures_util::StreamExt;
        
        let mut stream = Box::pin(stream);
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
            file.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;
            
            let _progress = if total_size > 0 {
                (downloaded as f32 / total_size as f32) * 100.0
            } else {
                0.0
            };
            // 这里可以发送进度更新，但为简单起见，暂时省略
        }
        
        file.flush().await?;
        
        Ok(DownloadProgress {
            file_name: task.file_name.clone(),
            total_size,
            downloaded_size: downloaded,
            progress: 100.0,
            status: "completed".to_string(),
        })
    }

    // 启动Aria2 RPC服务器
    pub async fn start_aria2_rpc(&self) -> Result<()> {
        // 检查Aria2是否已安装
        let output = Command::new("aria2c").arg("--version").output()?;
        if !output.status.success() {
            return Err(anyhow::anyhow!("Aria2未安装，请先安装Aria2"));
        }

        // 启动Aria2 RPC服务器
        let mut process_guard = self.aria2_process.lock().await;
        if process_guard.is_none() {
            let aria2_process = std::process::Command::new("aria2c")
                .args(&[
                    "--enable-rpc=true",
                    &format!("--rpc-listen-port={}", self.aria2_port),
                    "--rpc-allow-origin-all=true",
                    "--continue=true",
                    "--max-connection-per-server=8",
                    "--split=8",
                ])
                .spawn()?;
            *process_guard = Some(aria2_process);
        }

        // 等待Aria2启动
        sleep(Duration::from_secs(1)).await;

        Ok(())
    }

    // 停止Aria2 RPC服务器
    pub async fn stop_aria2_rpc(&self) -> Result<()> {
        // 发送停止命令到Aria2 RPC服务器
        let client = reqwest::Client::new();
        let rpc_url = format!("http://{}:{}/jsonrpc", self.aria2_host, self.aria2_port);
        
        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": "stop",
            "method": "aria2.shutdown"
        });
        
        let _response = client
            .post(&rpc_url)
            .header("Content-Type", "application/json")
            .body(request_body.to_string())
            .send()
            .await;
        
        // 无论RPC命令是否成功，都要确保进程被终止
        let mut process_guard = self.aria2_process.lock().await;
        if let Some(mut process) = process_guard.take() {
            // 尝试正常终止进程
            let _ = process.kill();
            let _ = process.wait();
        }
        
        Ok(())
    }
}

// 为Aria2 RPC功能添加辅助函数
pub mod aria2_rpc {
    use anyhow::Result;
    use serde_json::Value;
    use std::collections::HashMap;

    pub struct Aria2Client {
        rpc_url: String,
        client: reqwest::Client,
    }

    impl Aria2Client {
        pub fn new(host: &str, port: u16) -> Self {
            let rpc_url = format!("http://{}:{}/jsonrpc", host, port);
            Aria2Client {
                rpc_url,
                client: reqwest::Client::new(),
            }
        }

        pub async fn add_uri(&self, uris: Vec<String>, options: Option<HashMap<String, String>>) -> Result<String> {
            let mut params = serde_json::json!([uris]);
            if let Some(opts) = options {
                params = serde_json::json!([uris, opts]);
            }

            let request_body = serde_json::json!({
                "jsonrpc": "2.0",
                "id": "addUri",
                "method": "aria2.addUri",
                "params": params
            });

            let response = self.client
                .post(&self.rpc_url)
                .header("Content-Type", "application/json")
                .body(request_body.to_string())
                .send()
                .await?;

            let response_text = response.text().await?;
            let response_json: Value = serde_json::from_str(&response_text)?;

            if let Some(result) = response_json.get("result") {
                if let Some(gid) = result.as_str() {
                    Ok(gid.to_string())
                } else {
                    Err(anyhow::anyhow!("无法解析Aria2响应"))
                }
            } else {
                Err(anyhow::anyhow!("Aria2返回错误: {:?}", response_json.get("error")))
            }
        }

        pub async fn tell_status(&self, gid: &str) -> Result<Value> {
            let request_body = serde_json::json!({
                "jsonrpc": "2.0",
                "id": "tellStatus",
                "method": "aria2.tellStatus",
                "params": [gid]
            });

            let response = self.client
                .post(&self.rpc_url)
                .header("Content-Type", "application/json")
                .body(request_body.to_string())
                .send()
                .await?;

            let response_text = response.text().await?;
            let response_json: Value = serde_json::from_str(&response_text)?;

            if let Some(result) = response_json.get("result") {
                Ok(result.clone())
            } else {
                Err(anyhow::anyhow!("Aria2返回错误: {:?}", response_json.get("error")))
            }
        }
    }
}