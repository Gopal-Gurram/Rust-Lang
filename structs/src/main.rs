
// C-Like structs


    struct Color {
        red : u8,
        green : u8,
        blue : u8
    }

    fn main(){
        // creating an instance
        let black = Color{red:0,green:0,blue:0};
        println!("Black = rgb({},{},{})",black.red,black.green,black.blue);

        //Structs are immutable by default , if  use 'mut' to make it mutable but it doesn't support
        let mut link_color = Color{red:0,green:0,blue:255};
        link_color.blue = 238;
        println!("Link_Color= rgb({},{},{})",link_color.red,link_color.green,link_color.blue);

        // Copy elements from another instance
        let blue = Color {blue: 255, .. link_color};
        println!("Blue = rgb({}, {}, {})", blue.red, blue.green, blue.blue);

          // Destructure the instance using a `let` binding, this will not destruct blue instance
        let Color {red: r, green: g, blue: b} = blue;
        println!("Blue = rgb({}, {}, {})", r, g, b); //Blue = rgb(0, 0, 255)

          // Creating an instance via functions & accessing its fields
        let midnightblue = get_midnightblue_color();
        println!("Midnight Blue = rgb({}, {}, {})", midnightblue.red, midnightblue.green, midnightblue.blue); //Midnight Blue = rgb(25, 25, 112)

        // Destructure the instance using a `let` binding
        let Color {red: r, green: g, blue: b} = get_midnightblue_color();
        println!("Midnight Blue = rgb({}, {}, {})", r, g, b); //Midnight Blue = rgb(25, 25, 112)
    }

    fn get_midnightblue_color() -> Color {
        Color {red: 25, green: 25, blue: 112}
    }
