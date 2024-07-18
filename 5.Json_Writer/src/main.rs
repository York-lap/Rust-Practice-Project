/*
 * @Author: yk
 * @Date: 2024-07-17 10:12:24
 * @Description: This is a simple case to use serde_json crate to fish Struct -> Json
 */

use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
struct Paragraph{
    paragraph: String
}

#[derive(Serialize,Deserialize)]
struct Article{
    articel: String,
    author: String,
    paragraphs: Vec<Paragraph>
}


fn main() {

    let article: Article = Article{
        articel: String::from("Json-Writer"),
        author: String::from("york"),
        paragraphs: vec![
            Paragraph{
                paragraph: String::from("Head")
            },
            Paragraph{
                paragraph: String::from("Body")
            },
            Paragraph{
                paragraph: String::from("end")
            }
        ]
    };

    let json = serde_json::to_string(&article).unwrap();

    println!("{:#?}",json);


}