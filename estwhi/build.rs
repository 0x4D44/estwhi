use std::env;

fn main() {
    // Allow opting out of RC to unblock builds on machines without a working toolchain
    let skip = env::var("CARGO_FEATURE_NO_RES").is_ok()
        || env::var("ESTWHI_NO_RES").map(|v| v == "1").unwrap_or(false);
    if skip {
        println!("cargo:warning=Skipping resource compilation (feature no-res/ESTWHI_NO_RES=1)");
        return;
    }

    println!("cargo:rerun-if-changed=resources/app.rc");
    println!("cargo:rerun-if-changed=resources/cards.rcinc");
    println!("cargo:rerun-if-changed=resources/app.ico");
    println!("cargo:rerun-if-changed=assets");

    // Only attempt resource compilation when targeting Windows
    if env::var("CARGO_CFG_TARGET_OS")
        .map(|v| v != "windows")
        .unwrap_or(true)
    {
        println!("cargo:warning=Resource compilation skipped (target OS is not Windows)");
        return;
    }

    #[cfg(target_os = "windows")]
    {
        embed_resource::compile("resources/app.rc", embed_resource::NONE);
    }
}
