use colored::*;

pub fn print_banner() {
    let art = r#"
   ____                  _  ____  _____ ___ _   _ _______ 
  / __ \                (_)/ __ \| ____|_ _| \ | |__   __|
 | |  | |_ __ ___  _ __  _| |  | | (___ | ||  \| |  | |   
 | |  | | '_ ` _ \| '_ \| | |  | |\___ \| || . ` |  | |   
 | |__| | | | | | | | | | | |__| |____) | || |\  |  | |   
  \____/|_| |_| |_|_| |_|_|\____/|_____/___|_| \_|  |_|   
    "#;

    let subtitle = "       [ The All-Seeing Eye Framework v1.0 ]";
    let author =   "           Coded by NoHup-ltgm"; 

    println!("{}", art.bright_cyan().bold());
    println!("{}", subtitle.white().bold());
    println!("{}", author.red().italic()); 
    println!("\n{}", "-------------------------------------------------------".dimmed());
}