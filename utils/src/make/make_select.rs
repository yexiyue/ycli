use dialoguer::{Select, theme::ColorfulTheme};
pub fn make_select<T:std::fmt::Display>(prompt: &str,items:&Vec<T>)->usize
{
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(items)
        .interact()
        .unwrap()
}