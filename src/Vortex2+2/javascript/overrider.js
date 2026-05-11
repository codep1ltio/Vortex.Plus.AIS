const redirects = {
    ["vortex-engine.js"]: "vortex2+2_engine.js",
    ["multiplayer.js"]: "vortex2+2_multiplayer.js",
    ["parts.js"]: "vortex2+2_demoparts.js"

}


function local_thingy_platform_check(localPath) {
    const ua = navigator.userAgent.toLowerCase();
    if (ua.includes("windows") || ua.includes("android")) {
        return 'https://v22.' + localPath;
    }
    return 'v22://' + localPath;
};

let observer = new MutationObserver(function (mutations) {
    for (let m of mutations) {
        for (let node of m.addedNodes) {
            if (node.tagName === "SCRIPT") {
                let scriptName = node.src.split('/').pop().split('?')[0];
                if (redirects[scriptName]) {
                    node.src = local_thingy_platform_check('redirects/'+redirects[scriptName]);
                }
            }
        }
    }
});

observer.observe(document, {
    childList: true,
    subtree: true
});
