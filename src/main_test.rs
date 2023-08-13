#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let main = Main::new();
        assert_eq!(main.window_width, 1920);
        assert_eq!(main.window_height, 1080);
        assert_eq!(main.line_length, 0.0);
    }

    #[test]
    fn test_update() {
        let mut main = Main::new();
        main.update();
        assert_eq!(main.line_length, 2.0);
    }

    #[test]
    fn test_draw_stick() {
        let mut main = Main::new();
        main.draw_stick(10.0, 0.0, 0.0, 0.0, 1.0, 1.0);
        // TODO: assert that the line is drawn
    }
}
