use dialoguer::{Input, theme::ColorfulTheme};
pub fn make_input(prompt:&str)->String{
    Input::with_theme(&ColorfulTheme::default())
    .with_prompt(prompt)
    .validate_with(|input:&String|->Result<(), &str>{
        if input.len() > 0 {
            Ok(())
        }else{
            Err("Input cannot be empty")
        }
    })
    .interact_text()
    .unwrap()
}