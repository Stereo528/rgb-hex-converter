use std::fmt::{self, Formatter, Display};
use std::io;

struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for RGB {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\nRGB: {red}, {green}, {blue} {:<width$}->{:<width$} Hex: #{red:02X}{green:02X}{blue:02X}", "", "", red = self.red, green = self.green, blue = self.blue, width=4)
    }
}

fn main() {
    let mut hex_or_rgb = String::new();
    let mut rgb = String::new();
    let mut hex = String::new();

    println!("Enter 'hex' or 'rgb':");
    io::stdin().read_line(&mut hex_or_rgb).expect("Failed to read line");

    if hex_or_rgb.trim() == "hex" {
        println!("Enter Hex Value:");
        io::stdin().read_line(&mut hex).expect("Failed to read line");
        let hex_value = hex.trim().replace("#", "").replace(" ", "").replace("\"", "");
        for (ind, chr) in hex_value.chars().enumerate() {
            if !chr.is_digit(16) {
                println!("Invalid Hex Value");
                return;
            }
            if ind == 2 {
                let red = u8::from_str_radix(&hex_value[0..2], 16).unwrap();
                let green = u8::from_str_radix(&hex_value[2..4], 16).unwrap();
                let blue = u8::from_str_radix(&hex_value[4..6], 16).unwrap();
                println!("{}", RGB {red, green, blue});
            }
        }

    } else if hex_or_rgb.trim() == "rgb" {
        println!("Enter RGB Value");
        println!("NOTICE: Must be in the form of \"(000, 000, 000)\" (\"(, )\" optional):");
        io::stdin().read_line(&mut rgb).expect("Failed to read line");
        let rgb_value = rgb.trim().replace(" ", "").replace(",", "").replace("\"", "").replace("(", "").replace(")", "");
        if rgb_value.len() < 9 {
            println!("Invalid RGB Value, must be in the form of \"(000, 000, 000)\" (\"(, )\" optional)");
        }
        for (ind, chr) in rgb_value.chars().enumerate() {
            if !chr.is_digit(10) {
                println!("Invalid RGB Value");
                return;
            }
            if ind == 3 {
                let red = u8::from_str_radix(&rgb_value[0..3], 10).unwrap();
                let green = u8::from_str_radix(&rgb_value[3..6], 10).unwrap();
                let blue = u8::from_str_radix(&rgb_value[6..9], 10).unwrap();
                println!("{}", RGB {red, green, blue});
            }
        }
    } else {
        println!("Invalid Input, please input \"hex\" or \"rgb\"");
    }

}