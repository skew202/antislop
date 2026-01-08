// Dynamically load Mermaid.js from CDN and initialize it
(function () {
    var script = document.createElement('script');
    script.src = "https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.min.js";
    script.onload = function () {
        mermaid.initialize({
            startOnLoad: false,
            theme: 'dark',
            securityLevel: 'loose'
        });

        // Transform mdbook code blocks to mermaid divs
        var codes = document.querySelectorAll('code.language-mermaid');
        codes.forEach(function (code) {
            var pre = code.parentElement;
            var div = document.createElement('div');
            div.className = 'mermaid';
            div.textContent = code.textContent;

            // Replace pre with div
            if (pre && pre.tagName === 'PRE') {
                pre.replaceWith(div);
            }
        });

        // Run mermaid on the new divs
        mermaid.run();
    };
    document.head.appendChild(script);
})();
