pub fn get_diamond(c: char) -> Vec<String> {
    let base = c as u8 - 'A' as u8 + 1;
    let size = 2 * base - 1;
    let mut res = vec![];
    for i in 1..=size {
        let idx = if i <= base { i } else { 2 * base - i };
        let k = ('A' as u8 + (idx - 1)) as char;
        let mut row = vec![' '; size as usize];
        row[(size / 2 + (idx - 1)) as usize] = k;
        row[(size / 2 - (idx - 1)) as usize] = k;
        res.push(row.iter().collect());
    }
    res
}


mod diamond_test {
    use crate::diamond::get_diamond;

    #[test]
    fn a() {
        assert_eq!(get_diamond('A'), vec!["A"]);
    }

    #[test]
    fn b() {
        #[rustfmt::skip]
        assert_eq!(
            get_diamond('B'),
            vec![
                " A ",
                "B B",
                " A ",
            ]
        );
    }

    #[test]
    fn c() {
        #[rustfmt::skip]
        assert_eq!(
            get_diamond('C'),
            vec![
                "  A  ",
                " B B ",
                "C   C",
                " B B ",
                "  A  ",
            ]
        );
    }

    #[test]
    fn d() {
        #[rustfmt::skip]
        assert_eq!(
            get_diamond('D'),
            vec![
                "   A   ",
                "  B B  ",
                " C   C ",
                "D     D",
                " C   C ",
                "  B B  ",
                "   A   ",
            ]
        );
    }

    #[test]
    fn e() {
        assert_eq!(
            get_diamond('Z'),
            vec![
                "                         A                         ",
                "                        B B                        ",
                "                       C   C                       ",
                "                      D     D                      ",
                "                     E       E                     ",
                "                    F         F                    ",
                "                   G           G                   ",
                "                  H             H                  ",
                "                 I               I                 ",
                "                J                 J                ",
                "               K                   K               ",
                "              L                     L              ",
                "             M                       M             ",
                "            N                         N            ",
                "           O                           O           ",
                "          P                             P          ",
                "         Q                               Q         ",
                "        R                                 R        ",
                "       S                                   S       ",
                "      T                                     T      ",
                "     U                                       U     ",
                "    V                                         V    ",
                "   W                                           W   ",
                "  X                                             X  ",
                " Y                                               Y ",
                "Z                                                 Z",
                " Y                                               Y ",
                "  X                                             X  ",
                "   W                                           W   ",
                "    V                                         V    ",
                "     U                                       U     ",
                "      T                                     T      ",
                "       S                                   S       ",
                "        R                                 R        ",
                "         Q                               Q         ",
                "          P                             P          ",
                "           O                           O           ",
                "            N                         N            ",
                "             M                       M             ",
                "              L                     L              ",
                "               K                   K               ",
                "                J                 J                ",
                "                 I               I                 ",
                "                  H             H                  ",
                "                   G           G                   ",
                "                    F         F                    ",
                "                     E       E                     ",
                "                      D     D                      ",
                "                       C   C                       ",
                "                        B B                        ",
                "                         A                         ",
            ]
        );
    }
}