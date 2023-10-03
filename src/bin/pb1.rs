

fn main() {

    let data = [
        "<--",
        "****5",
        "<=="
    ];

    let data = data.map(|x| x.to_string());

    let res = vflip(&data);

    let res = hflip(&data);
    println!("{:?}", res);

}


fn vflip( img : &[String]) -> Vec<String> {

    let mut result = Vec::new();

    for i in (0..img.len()).rev() {
        
        result.push( img[ i ].to_string() );
    }

    result
}


fn hflip( img : &[String]) -> Vec<String> {

    let mut result = Vec::new();

    let mut longest_len = 0;

    for i in 0..img.len() {

        if img[i].len() > longest_len {
            longest_len = img[i].len();
        }
    }

    for i in 0..img.len() {
        
        let text = &img[i].clone();
        let text: String = text.chars().rev().collect();
        let mut head = String::new();

        for _ in 0..(longest_len - text.len()) {
            head.push_str(" ");
        }

        result.push( format!("{}{}", head, text) );
        
    }
    result
}





#[test]
fn test_img_flip() {
    let emp = ["".to_string(); 0];
    assert_eq!(vflip(&emp), [""; 0]);
    assert_eq!(hflip(&emp), [""; 0]);
    let data = [
        "<--",
        "#####",
        "<=="
    ].map(|v| v.to_string());

    assert_eq!(
    vflip(&data), [
        "<==",
        "#####",
        "<--"
    ]);
    assert_eq!(
    hflip(&data), [
        "  --<",
        "#####",
        "  ==<"
    ]);
}