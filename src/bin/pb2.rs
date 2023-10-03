

fn main() {

    let data = [
        "<--",
        "#####",
        "<=="
    ];

    let data = data.map(|x| x.to_string());

    let res = vcat(&data, &data);
    println!("{:?}", res);
    let res = hcat(&data, &data[..2]);
    println!("{:?}", res);
    let res = hcat(&data[..2], &data);
    println!("{:?}", res);

}


fn vcat( img1 : &[String], img2 : &[String]) -> Vec<String> {

    let mut result = Vec::new();

    for i in 0..img1.len() {
        
        result.push( img1[ i ].to_string() );
    }

    for i in 0..img2.len() {
        
        result.push( img2[ i ].to_string() );
    }

    result
}


fn hcat( img1 : &[String], img2 : &[String]) -> Vec<String> {

    let mut result: Vec<String> = Vec::new();

    let mut longest_len1 = 0;
    for i in 0..img1.len() {
        if img1[i].len() > longest_len1 {
            longest_len1 = img1[i].len();
        }
    }

    for i in 0..img1.len() {

        result.push(img1[i].clone());
    }

    for i in 0..img2.len() {

        let text= img2[i].clone();
        let mut space = String::new();

        if i < result.len() {

            for _ in 0..(longest_len1 - (result[i].len())) {
                space.push_str(" ");
            }
            result[i].push_str(format!("{}{}", space, img2[i]).as_str());
        }else {
            for _ in 0..(longest_len1) {
                space.push_str(" ");
            }
            result.push(format!("{}{}", space, img2[i]));
        }
        
    }

    result
}



#[test]
fn test_img_cat() {
    let emp = ["".to_string(); 0];
    assert_eq!(vcat(&emp, &emp), [""; 0]);
    assert_eq!(hcat(&emp, &emp), [""; 0]);
    let data = [
        "<--",
        "#####",
        "<=="
    ].map(|v| v.to_string());
    assert_eq!(vcat(&emp, &data), data);
    assert_eq!(vcat(&data, &emp), data);
    assert_eq!(
    vcat(&data, &data), [
        "<--",
        "#####",
        "<==",
        "<--",
        "#####",
        "<=="
    ]);
    assert_eq!(
    hcat(&data, &data[..2]), [
        "<--  <--",
        "##########",
        "<=="
    ]);
    assert_eq!(
        hcat(&data[..2], &data), [
        "<--  <--",
        "##########",
        "     <=="
    ]);
}