extern crate libc;


use regex::Regex;


fn main() {
    let info = unsafe {
        use std::mem::MaybeUninit;
        let mut buff : MaybeUninit<libc::utsname> = MaybeUninit::uninit(); 
        libc::uname(buff.as_mut_ptr());
        buff.assume_init()
    };

    let hostname = cstr_to_str(&info.nodename);
    let kernel_ver = cstr_to_str(&info.release);

    let user_name = std::env::var("USER").unwrap();
    let shell_name = std::env::var("SHELL").unwrap();

    let name_field_reg = Regex::new(r"^NAME=").unwrap();

    let os_info = std::fs::read_to_string("/etc/os-release").unwrap();

    let lines : Vec<&str> = os_info.split("\n").collect();

    let mut os_name : String = "<unknown>".to_string();

    for line in lines.iter() {
        if name_field_reg.is_match(line) {
            os_name = line.to_string();
            os_name = os_name.strip_prefix("NAME=\"").unwrap().strip_suffix("\"").unwrap().to_string();
        }
    };

    //  _n_ 
    //  |'> 
    //  |U|
    //  /_|
    //   "

    // Black	30	40
    // Red	    31	41
    // Green	32	42
    // Yellow	33	43
    // Blue	    34	44
    // Magenta	35	45
    // Cyan	    36	46
    // White	37	47

    let hat_color = color_code(match rand::random::<u32>() % 5 {
        0 => Color::Red,
        1 => Color::Green,
        2 => Color::Blue,
        3 => Color::Magenta,
        _ => Color::Cyan
    });
    let eye_color = color_code(Color::Green);
    let beak_color = color_code(Color::Yellow);
    let feet_color = color_code(Color::Yellow);
    let reset_color = reset_code();

    let palette = (0..8).map(|c| format!("\x1b[{}m  ",c + 40)).fold("".to_string(), |v,acc| acc + &v) + &reset_color;

    println!(" {hat_color}_n_  {reset_color}|{: ^18}|",
             user_name + "@" + &hostname,hat_color=hat_color,reset_color=reset_color);
    println!(" |{eye_color}'{beak_color}>  {reset_color}|{: ^18}|" ,
             os_name,beak_color=beak_color,reset_color=reset_color,eye_color = eye_color);
    println!(" |U|  |{: ^18}|",kernel_ver);
    println!(" /_|  |{: ^18}|",shell_name);
    println!("  {feet_color}\"   {reset_color}| {} |",palette,feet_color=feet_color,reset_color=reset_color);

}


fn cstr_to_str(arr:&[i8;65]) -> String {
    arr.iter().filter(|&&a| a != 0).map(|&a| a as u8 as char).collect::<String>()
}

#[derive(Clone,Copy)]
#[allow(dead_code)]
enum Color {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
}

fn color_code(fg: Color) -> String {
    format!("\x1b[{}m",fg as i8 + 30).to_string()
}

fn reset_code() -> String {
    "\x1b[0m".to_string()
}
