//! A module that handles the view for the settings page in the `websurfx` frontend.

use maud::{html, Markup};

use crate::templates::partials::{
    footer::footer,
    header::header,
    settings_tabs::{
        cookies::cookies, engines::engines, general::general, user_interface::user_interface,
    },
};

/// A function that handles the html code for the settings page view in the search engine frontend.
///
/// # Arguments
///
/// * `colorscheme` - It takes the colorscheme name as an argument.
/// * `theme` - It takes the theme name as an argument.
/// * `engine_names` - It takes a list of engine names as an argument.
///
/// # Error
///
/// This function returns a compiled html markup code on success otherwise returns a standard error
/// message.
pub fn settings(
    colorscheme: &str,
    theme: &str,
    animation: &Option<String>,
    engine_names: &[&String],
) -> Result<Markup, Box<dyn std::error::Error>> {
    Ok(html!(
        (header(colorscheme, theme, animation))
        main class="settings"{
           h1{"Settings"}
           hr;
           .settings_container{
              .sidebar{
                  div class="btn active" onclick="setActiveTab(this)"{"general"}
                  .btn onclick="setActiveTab(this)"{"user interface"}
                  .btn onclick="setActiveTab(this)"{"engines"}
                  .btn onclick="setActiveTab(this)"{"cookies"}
              }
              .main_container{
                  (general())
                  (user_interface()?)
                  (engines(engine_names))
                  (cookies())
                  p class="message"{}
                  button type="submit" onclick="setClientSettings()"{"Save"}
              }
           }
        }
        script src="static/settings.js"{}
        script src="static/cookies.js"{}
        (footer())
    ))
}
