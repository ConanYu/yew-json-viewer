
export function copy_text(text) {
    navigator.clipboard.writeText(text).then(() => {});
}
export function set_body_overflow_style(overflow) {
    document.body.style.overflow = overflow;
}
