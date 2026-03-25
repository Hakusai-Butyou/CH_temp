fn main() {
    // .envファイルを読み込む
    if let Ok(path) = dotenvy::dotenv() {
        // .envが変更されたら再ビルドするように指示
        println!("cargo:rerun-if-changed={}", path.display());

        // 各変数をrustc-envとして出力し、ビルド時に利用可能にする
        for item in dotenvy::vars() {
            let (key, value) = item;
            println!("cargo:rustc-env={}={}", key, value);
        }
    }
}