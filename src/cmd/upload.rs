use std::path::{Path, PathBuf};

use clap::Parser;
use reqwest::{header::AUTHORIZATION, multipart, multipart::Part, Body, StatusCode, Url};
use sc_cli::{utils, KeystoreParams};
use sha2::{Digest, Sha256};
use sp_core::sr25519::Pair;
use tokio::{fs::File, io::AsyncReadExt};
use tokio_util::io::ReaderStream;

use crate::{chain::Client, jwt::generate_token, AgreementId, Timestamp};

const JWT_TTL: Timestamp = 24 * 60 * 60 * 1000;

/// The `upload` command
#[derive(Debug, Clone, Parser)]
#[command(
    name = "upload",
    about = "Upload a file to the chain, with a given (secret) key"
)]
pub struct UploadCmd {
    /// The file to upload.
    #[arg(long, value_name = "PATH")]
    file_path: PathBuf,

    /// The node URL.
    #[arg(long, value_name = "URL")]
    deitos_url: String,

    /// The Infrastructure Provider URL.
    #[arg(long, value_name = "URL")]
    ip_url: String,

    /// The agreement id.
    #[arg(long)]
    agreement: AgreementId,

    /// The secret key URI.
    /// If the value is a file, the file content is used as URI.
    /// If not given, you will be prompted for the URI.
    #[arg(long)]
    suri: Option<String>,

    #[allow(missing_docs)]
    #[clap(flatten)]
    keystore_params: KeystoreParams,
}

impl UploadCmd {
    fn get_keypair(&self) -> Pair {
        let suri = utils::read_uri(self.suri.as_ref()).expect("Secret key URI should be valid");
        let password = self
            .keystore_params
            .read_password()
            .expect("Password should be valid");

        utils::pair_from_suri::<Pair>(&suri, password).expect("Pair should be valid")
    }

    async fn get_file_hash(&self) -> String {
        let file_hash = calculate_file_hash(&self.file_path)
            .await
            .expect("File hash should calculate successfully");
        println!("File hash: {}", file_hash);
        file_hash
    }

    /// Run the upload command
    pub async fn run(&self) {
        let pair = self.get_keypair();
        let file_hash = self.get_file_hash().await;
        let ip_url = Url::parse(&self.ip_url).expect("IP URL should be valid");

        let client = Client::new(&self.deitos_url).await;
        client
            .register_file(
                pair.clone(),
                self.agreement,
                file_hash,
                get_file_name(&self.file_path),
            )
            .await;

        let timestamp = client
            .get_timestamp()
            .await
            .expect("Timestamp should be available");
        let jwt = generate_token(pair, self.agreement, timestamp, timestamp + JWT_TTL);
        upload_file(&self.file_path, &ip_url, &jwt).await;
    }
}

async fn upload_file(file_path: &Path, ip_url: &Url, jwt: &str) {
    let url = ip_url.join("/v1/files").expect("IP URL should be valid");
    println!("Uploading file to {url} with JWT {jwt}");

    let file = File::open(file_path)
        .await
        .expect("File should open successfully");
    let stream = ReaderStream::new(file);
    let file_part = Part::stream(Body::wrap_stream(stream)).file_name(get_file_name(file_path));
    let form = multipart::Form::new().part("file", file_part);

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header(AUTHORIZATION, format!("Bearer {jwt}"))
        .multipart(form)
        .send()
        .await
        .expect("File should upload successfully");

    if response.status() == StatusCode::OK {
        println!("File uploaded successfully");
    } else {
        println!(
            "File upload failed: {}",
            response.text().await.expect("Response should have text")
        );
    }
}

fn get_file_name(path: &Path) -> String {
    path.file_name()
        .expect("File path should contain a file name")
        .to_str()
        .expect("File name should be valid")
        .to_owned()
}

async fn calculate_file_hash(file_path: &Path) -> Result<String, std::io::Error> {
    let mut digest = Sha256::new();
    let mut file = File::open(file_path).await?;

    let mut buffer = [0; 8 * 1024];
    loop {
        let size = file.read(&mut buffer[..]).await?;
        if size == 0 {
            break;
        }

        digest.update(&buffer[0..size]);
    }

    let hash = digest.finalize();
    Ok(format!("{:x}", hash))
}
