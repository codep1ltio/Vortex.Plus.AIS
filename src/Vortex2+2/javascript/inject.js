function ensureCSS(css) {
    if (document.getElementById("__vortex_css__")) return;

    const style = document.createElement("style");
    style.id = "__vortex_css__";
    style.textContent = css;

    (document.head || document.documentElement).appendChild(style);
}

function inject(css) {
    const apply = () => ensureCSS(css);

    apply();

    const observer = new MutationObserver(apply);

    observer.observe(document.documentElement, {
        childList: true,
        subtree: true
    });
}

function run(css) {
    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", () => inject(css));
    } else {
        inject(css);
    }
}