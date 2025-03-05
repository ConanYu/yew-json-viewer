
export function with_global_javascript_interaction() {
    return !!window.__yew_json_viewer__interaction;
}
export function do_global_javascript_interaction(json_string) {
    return window.__yew_json_viewer__interaction(json_string);
}
