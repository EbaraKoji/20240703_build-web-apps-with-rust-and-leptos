use leptos::*;
use leptos_router::ActionForm;

use crate::models::color::Color;
use crate::repositories::color::AppendColor;

#[component]
pub fn ColorForm(append_color: Action<AppendColor, Result<Color, ServerFnError>>) -> impl IntoView {
    view! {
        <ActionForm action=append_color>
            <label for="">"Name: "<input type="text" name="name" /></label>
            <label for="">"Hex Code: "<input type="text" name="hex_code" /></label>
            <button type="submit">"Add Color"</button>
        </ActionForm>
    }
}
