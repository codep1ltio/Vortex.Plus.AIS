const redirects = {

    ["vortex-engine.js"]: "new_engine.js",

}


function local_thingy_platform_check(localPath) {
    const ua = navigator.userAgent.toLowerCase();
    if (ua.includes("windows") || ua.includes("android")) {
        return 'https://local.' + localPath;
    }
    return 'local://' + localPath;
};

let observer = new MutationObserver(function (mutations) {
    for (let m of mutations) {
        for (let node of m.addedNodes) {
            if (node.tagName === "SCRIPT") {
                let divided = node.src.split('/');
                let scriptName = divided[divided.length - 1]
                if (redirects[scriptName]) {
                    node.src = local_thingy_platform_check(redirects[scriptName]);
                }
            }
        }
    }
});

observer.observe(document, {
    childList: true,
    subtree: true
});
