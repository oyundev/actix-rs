#![deny(warnings)]
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use chrono::{DateTime, Utc};

#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[get("/")]
async fn index_default() -> impl Responder {
    // let txt = get_news_data().await;
    format!("hello \n \tsecondline \n{}", 2)
}
 
#[get("/test")]
async fn index_test(_req: HttpRequest) -> HttpResponse {
    //HttpResponse::Ok().json("Hello world!")
    //HttpResponse::Ok().body("<b>Hello</b> <i>world!</i>")
    let txt = get_news_data().await;
    HttpResponse::Ok().body(txt)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(index_default)
            .service(index_test)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn get_news_data() -> String {
/*
╔══════════════════════════════════════════════════════════════════════════════╗
║ html static parts, tags, styles and scripts                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝
*/
let html_part_head = r#"
    <html><head>
    <title>Son Dakika Haberleri</title>
    <meta http-equiv="Content-Type" content="text/html; charset=UTF-8;">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <style>
    .collapsible {
      background-color: #777;
      color: white;
      cursor: pointer;
      padding: 18px;
      width: 100%;
      border: none;
      text-align: left;
      outline: none;
      font-size: 17px;
    }
    
    .active, .collapsible:hover {
      background-color: #483855;
    }
    
    .content {
      padding: 0 18px;
      display: none;
      overflow: hidden;
      font-size: 17px;
      font-family: arial;
      background-color: #f1fff2;
    }
    </style>
    </head>
    <body style="background-color: #efefef;">    
    <h2><u>Son Dakika Haberleri</u></h2>     
    
"#;
//
let html_part_button_1 = r#"
<button type="button" class="collapsible">
"#;
//
let html_part_button_2 = r#"
</button>
<div class="content">
  <pre>
"#;
//
let html_part_button_3 = r#"
</pre>
</div>
"#;
//
let html_part_end = r#"
 
<script>
var coll = document.getElementsByClassName("collapsible");
var i;

for (i = 0; i < coll.length; i++) {
  coll[i].addEventListener("click", function() {
    this.classList.toggle("active");
    var content = this.nextElementSibling;
    if (content.style.display === "block") {
      content.style.display = "none";
    } else {
      content.style.display = "block";
    }
  });
}
</script>

</body>
</html>
"#;
    /*
    ╔══════════════════════════════════════════════════════════════════════════════╗
    ║ use reqwest client to get page content with proper character encoding        ║
    ╚══════════════════════════════════════════════════════════════════════════════╝
    */
    let client = reqwest::Client::new();
    //
    let mut result_string: String = "".to_owned();
    result_string.push_str(html_part_head);
    //
    let res = client
        .get(" ~~~~~~~~~~ redacted-url-1 ~~~~~~~~~~ ")
        .header("Accept", "text/html;charset=windows-1254")
        .header("Content-Type", "text/html; charset=windows-1254") 
        .send()
        .await
        .unwrap()
        .text_with_charset("windows-1254")
        .await
        .unwrap();
        //.header(Authorization, "Basic abc456hghtyhg==")
        //.header("Content-Type", "application/x-www-form-urlencoded")
        //.header(grant_type, "client_credentials")
    /*
    ╔══════════════════════════════════════════════════════════════════════════════╗
    ║ parse dom document and iterate content to find news head-lines and detail-url║
    ╚══════════════════════════════════════════════════════════════════════════════╝
    */
    for node in Document::from(res.as_str())
        .find(Class("anatxt").descendant(Name("a")))
        .take(5)
    {
        let url_desc = node.text();
        let url_pid = node.attr("href").unwrap();
        let url_detail = url_pid
            .replace(
                "javascript:openWindow('",
                " ~~~~~~~~~~ redacted-url-2 ~~~~~~~~~~ ",
            )
            .replace("');", "");
        //
        result_string.push_str(html_part_button_1);
        result_string.push_str(&url_desc);
        result_string.push_str(html_part_button_2);
        /*
        ╔══════════════════════════════════════════════════════════════════════════════╗
        ║ get news details using reqwest client and extact detail content using DOM    ║
        ╚══════════════════════════════════════════════════════════════════════════════╝
        */
        let res_detail = client
            .get(&url_detail)
            .header("Accept", "text/html;charset=windows-1254")
            .header("Content-Type", "text/html; charset=windows-1254")
            .send()
            .await
            .unwrap()
            .text_with_charset("windows-1254")
            .await
            .unwrap();
        //
        for node_detail in Document::from(res_detail.as_str())
            .find(Class("anatxt"))
            .take(1)
        {
            let detail_desc = node_detail.text();
         
            result_string.push_str(&detail_desc);
            result_string.push_str(html_part_button_3);
        }
    }
    /*
    ╔══════════════════════════════════════════════════════════════════════════════╗
    ║ finalize page content add date and time stamp UTC bottom of the page         ║
    ╚══════════════════════════════════════════════════════════════════════════════╝
    */
    let now: DateTime<Utc> = Utc::now();
    let datetimestring = format!("<br><hr><br><pre>UTC now in RFC 2822 is: {}</pre><br><hr>", now.to_rfc2822());
    //
    result_string.push_str(&datetimestring);
    result_string.push_str(html_part_end);
    //
    return result_string;
}
