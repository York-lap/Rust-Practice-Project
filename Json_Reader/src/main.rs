/*
 * @Author: yk
 * @Date: 2024-07-16 15:17:06
 * @Description: This is a simple case to read Json & [Serialize + Deserialize]
 */
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
struct Paragraph{
    name: String,
}

#[derive(Serialize,Deserialize)]
struct Article{
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {

    // 1. Initial a json data
    let json = r#"
    {
        "article":"How to work with json in rust",
        "author":"york",
        "paragraph":[
            {
                "name": "Title"
            },
            {
                "name": "Midle"
            },
            {
                "name": "End"
            }
        ]
    }"#;

    let parsed: Article = read_json_typed(json);
     
    println!("The Second Paragraph is: {:?}",parsed.paragraph[0].name);

}

fn read_json_typed(raw_json: &str) -> Article{
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed
}
