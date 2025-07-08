use age::{Decryptor, Encryptor};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use rpassword::prompt_password;
use secrecy::SecretString;
use std::fs::File;
use std::io::{self, Read};

#[derive(Parser)]
#[command(name = "my tool")]
#[command(about = "Инструмент шифрования файлов")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encrypt { input: String, output: String },
    Decrypt { input: String, output: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encrypt { input, output } => {
            let passphrase = prompt_password("Введите пароль: ")?;
            let passphrase = SecretString::new(passphrase);
            let mut input_file = File::open(input).context("Не удалось открыть входной файл")?;
            let mut output_file = File::create(output).context("Не удалось создать выходной файл")?;
            let encryptor = Encryptor::with_user_passphrase(passphrase);
            let mut writer = encryptor.wrap_output(&mut output_file)?;
            io::copy(&mut input_file, &mut writer).context("Не удалось зашифровать файл")?;
            writer.finish().context("Не удалось завершить шифрование")?;
        }
        Commands::Decrypt { input, output } => {
            let passphrase = prompt_password("Введите пароль: ")?;
            let passphrase = SecretString::new(passphrase);
            let mut input_file = File::open(input).context("Не удалось открыть входной файл")?;
            let mut output_file = File::create(output).context("Не удалось создать выходной файл")?;
            let mut encrypted_data = Vec::new();
            input_file
                .read_to_end(&mut encrypted_data)
                .context("Не удалось прочитать входной файл")?;
            let decrypt_enum = Decryptor::new(&encrypted_data[..]).context("Не удалось создать расшифровщик")?;
            let mut reader = match decrypt_enum {
                Decryptor::Passphrase(d) => d.decrypt(&passphrase.clone(), None)?,
                _ => return Err(anyhow::anyhow!("Expected passphrase-based encryption")),
            };
            io::copy(&mut reader, &mut output_file).context("Не удалось расшифровать файлъ")?;
        }
    }

    Ok(())
}