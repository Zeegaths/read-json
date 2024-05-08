use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String
}

#[derive(Serialize, Deserialize)]

//the vector hold multiple paragraphs
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() {
    let json = r#"
    {
        "article": "Progressive learning of rust",
        "author": "Mary",
        "paragraph": [
            {
                "name": "Heading 1"
            },
            {
                "name": "Heading 2"
            },
            {
                "name": "Heading 3"
            }
        ]
    }"#;

    let parsed: Article = read_json_typed(json);
    println!("\n\n The name of the first paragraph is : {}", parsed.paragraph[0].name);
    println!("The paragraph itself is: {}", parsed.article);

}

fn read_json_typed(raw_json : &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;

} 