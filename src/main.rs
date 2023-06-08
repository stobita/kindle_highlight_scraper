
#[tokio::main]
async fn  main() {
    //https://read.amazon.co.jp/notebook をスクレイピング
    let url = "https://read.amazon.co.jp/notebook";
    let mut res = reqwest::get(url).await.unwrap();
    assert!(res.status().is_success());
    // すべての本のurlを取得
    // id=libraryの中のclass=kp-notebook-library-each-bookのidを取得
    // for node in document.find(Attr("id", "library").descendant(Class("kp-notebook-library-each-book"))) {
    //     println!("node = {:?}", node);
    //     // 本のurlを取得
    //     let mut url = node.find(Name("a")).next().unwrap();
    //     println!("url = {:?}", url);
    //     // 本のタイトルを取得
    //     let mut title = node.find(Class("kp-notebook-library-each-book-title")).next().unwrap();
    //     println!("title = {:?}", title);
    //     // 本の著者を取得
    //     let mut author = node.find(Class("kp-notebook-library-each-book-author")).next().unwrap();
    //     println!("author = {:?}", author);
    //     // 本の画像を取得
    //     let mut image = node.find(Class("kp-notebook-library-each-book-image")).next().unwrap();
    //     println!("image = {:?}", image);
    // }
}
