use std::fs::File;
use std::io::Write;
use transformrs::text_to_image::TTIConfig;
use transformrs::Provider;

#[derive(clap::Parser)]
pub(crate) struct TextToImageArgs {
    /// Model to use (optional)
    #[arg(long)]
    model: Option<String>,

    /// Number of steps (optional)
    #[arg(long, default_value_t = 10)]
    steps: u32,

    /// CFG scale (optional)
    #[arg(long, default_value_t = 3)]
    cfg_scale: u32,

    /// Height (optional)
    #[arg(long, default_value_t = 512)]
    height: u32,

    /// Width (optional)
    #[arg(long, default_value_t = 512)]
    width: u32,

    /// Output filename without extension
    #[arg(long, short = 'o')]
    output: Option<String>,
}

fn default_model(provider: &Provider) -> String {
    match provider {
        Provider::DeepInfra => "black-forest-labs/FLUX-1-schnell".to_string(),
        _ => "black-forest-labs/FLUX-1-dev".to_string(),
    }
}

pub(crate) async fn tti(args: &TextToImageArgs, key: &transformrs::Key, input: &str) {
    let provider = key.provider.clone();
    let config = TTIConfig {
        model: args
            .model
            .clone()
            .unwrap_or_else(|| default_model(&provider)),
        steps: Some(args.steps),
        cfg_scale: Some(args.cfg_scale),
        height: Some(args.height),
        width: Some(args.width),
    };
    let resp = transformrs::text_to_image::text_to_image(key, config, input)
        .await
        .unwrap()
        .structured()
        .unwrap();
    let encoded = &resp.images[0];
    let image = encoded.base64_decode().unwrap();
    if let Some(output) = &args.output {
        let filename = format!("{}.{}", output, image.filetype);
        let mut file = File::create(filename).unwrap();
        file.write_all(&image.image).unwrap();
    } else {
        std::io::stdout().write_all(&image.image).unwrap();
    }
}
