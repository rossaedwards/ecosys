// src/tts/tts_engine.rs

//! TTS Engine for AuraFS Holographic Logger and UI
//! Supports async text input, multi-voice output, and streaming playback

use anyhow::{Context, Result};
use async_trait::async_trait;
use tokio::process::Command;

#[derive(Clone, Debug)]
pub enum TtsVoice {
    Default,
    Ross,
    Audry,
    QuantumAura,
}

#[async_trait]
pub trait Tts {
    async fn speak(&self, text: &str) -> Result<()>;
}

pub struct TtsEngine {
    voice: TtsVoice,
}

impl TtsEngine {
    pub fn new(voice: TtsVoice) -> Result<Self> {
        Ok(Self { voice })
    }

    #[cfg(target_os = "windows")]
    fn windows_voice_arg(&self) -> &'static str {
        // Standard Windows SAPI voices
        match self.voice {
            TtsVoice::Default => "Microsoft Zira Desktop",
            TtsVoice::Ross => "Microsoft Zira Desktop",
            TtsVoice::Audry => "Microsoft Zira Desktop",
            TtsVoice::QuantumAura => "Microsoft Zira Desktop",
        }
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn unix_voice_arg(&self) -> &'static str {
        match self.voice {
            TtsVoice::Default => "Alex",
            TtsVoice::Ross => "Alex",
            TtsVoice::Audry => "Alex",
            TtsVoice::QuantumAura => "Alex",
        }
    }
}

#[async_trait]
impl Tts for TtsEngine {
    async fn speak(&self, text: &str) -> Result<()> {
        // Call system TTS engine or external process asynchronously
        // Using PowerShell with SAPI on Windows, `say` command on macOS/Linux
        #[cfg(target_os = "windows")]
        {
            // Escape single quotes in text for PowerShell
            let escaped_text = text.replace("'", "''");
            let voice = self.windows_voice_arg();
            
            // Use proper PowerShell escaping with -Command parameter
            let ps_script = format!(
                "Add-Type -AssemblyName System.speech; $speak = New-Object System.Speech.Synthesis.SpeechSynthesizer; try {{ $speak.SelectVoice('{}') }} catch {{ }}; $speak.Speak('{}')",
                voice, escaped_text
            );

            let output = Command::new("powershell")
                .arg("-Command")
                .arg(ps_script)
                .output()
                .await
                .context("Failed to execute PowerShell TTS command")?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(anyhow::anyhow!(
                    "TTS command failed: {}",
                    stderr
                ));
            }
        }

        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            let output = Command::new("say")
                .arg("-v")
                .arg(self.unix_voice_arg())
                .arg(text)
                .output()
                .await
                .context("Failed to execute 'say' TTS command")?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(anyhow::anyhow!(
                    "TTS command failed: {}",
                    stderr
                ));
            }
        }

        Ok(())
    }
}