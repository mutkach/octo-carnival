#[cfg(test)]
mod tests {
    use crate::color::{Color, ColorMap};

    #[test]
    fn check_hexcolor_0() {
        let hex = "#00ff01";
        let color = Color::new((0, 255, 1, 255));
        let hexcolor = Color::from_hex(hex);
        match hexcolor {
            Ok(hexcolor) => assert_eq!(hexcolor, color),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn check_hexcolor_1() {
        let hex = "0x00ff01";
        let color = Color::new((0, 255, 1, 255));
        let hexcolor = Color::from_hex(hex);
        match hexcolor {
            Ok(hexcolor) => assert_eq!(hexcolor, color),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn check_colormap_0() {
        let filename = "./www/color.conf";
        let result = ColorMap::from_conf(filename);
        assert!(result.is_ok())
    }

    #[test]
    fn check_colormap_1() {
        let filename = "./www/color.conf";
        let result = ColorMap::from_conf(filename).expect("Wrong format");
        let color = result.get("background").unwrap().to_owned();
        assert_eq!(color, Color::from_hex("#000000").unwrap());
    }
}
