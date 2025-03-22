use image::RgbImage;

fn main() {
    let mut img = image::open("test.jpg").unwrap().to_rgb8();
    let message = "Hide this for me !";
    hide_message(&mut img, message);
    let msg = extract_message(&img, message.len()); 
    println!("Found: {}", msg); 
    }

fn hide_message(image: &mut RgbImage, message: &str) {
    let mut message_bits = message.chars().flat_map(|c| {
        format!("{:08b}", c as u8).chars().collect::<Vec<char>>()
    });


    for pixel in image.pixels_mut() {
        if let Some(bit) = message_bits.next() {
            let red = pixel.0[0] & 0b11111110 | bit.to_digit(2).unwrap() as u8;
            pixel.0[0] = red;
        }
        if let Some(bit) = message_bits.next() {
            let green = pixel.0[1] & 0b11111110 | bit.to_digit(2).unwrap() as u8;
            pixel.0[1] = green;
        }
        if let Some(bit) = message_bits.next() {
            let blue = pixel.0[2] & 0b11111110 | bit.to_digit(2).unwrap() as u8;
            pixel.0[2] = blue;
        }
    }

}

fn extract_message(image: &RgbImage, message_length: usize) -> String {
    let mut message_bits = Vec::new();
    for pixel in image.pixels() {
        message_bits.push((pixel.0[0] & 1).to_string());
        message_bits.push((pixel.0[1] & 1).to_string());
        message_bits.push((pixel.0[2] & 1).to_string());
        if message_bits.len() >= message_length * 8 {
            break;
        }
    }

    let message_bits = message_bits.concat();
    let mut message = String::new();
    for i in (0..message_bits.len()).step_by(8) {
        if i + 8 <= message_bits.len() {
            let byte = u8::from_str_radix(&message_bits[i..i+8], 2).unwrap();
            message.push(byte as char);
        }
    }
    message

}
