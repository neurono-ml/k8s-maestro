// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="index.html"><strong aria-hidden="true">1.</strong> Introduction</a></li><li class="chapter-item expanded "><a href="getting-started/index.html"><strong aria-hidden="true">2.</strong> Getting Started</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="getting-started/installation.html"><strong aria-hidden="true">2.1.</strong> Installation</a></li><li class="chapter-item expanded "><a href="getting-started/quick-start.html"><strong aria-hidden="true">2.2.</strong> Quick Start</a></li><li class="chapter-item expanded "><a href="getting-started/concepts.html"><strong aria-hidden="true">2.3.</strong> Concepts</a></li></ol></li><li class="chapter-item expanded "><a href="guides/index.html"><strong aria-hidden="true">3.</strong> Guides</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="guides/basic-workflow.html"><strong aria-hidden="true">3.1.</strong> Basic Workflow</a></li><li class="chapter-item expanded "><a href="guides/k8s-job-step.html"><strong aria-hidden="true">3.2.</strong> KubeJobStep</a></li><li class="chapter-item expanded "><a href="guides/k8s-pod-step.html"><strong aria-hidden="true">3.3.</strong> KubePodStep</a></li><li class="chapter-item expanded "><a href="guides/containers.html"><strong aria-hidden="true">3.4.</strong> Containers</a></li><li class="chapter-item expanded "><a href="guides/volumes.html"><strong aria-hidden="true">3.5.</strong> Volumes</a></li><li class="chapter-item expanded "><a href="guides/networking.html"><strong aria-hidden="true">3.6.</strong> Networking</a></li><li class="chapter-item expanded "><a href="guides/security.html"><strong aria-hidden="true">3.7.</strong> Security</a></li><li class="chapter-item expanded "><a href="guides/observers.html"><strong aria-hidden="true">3.8.</strong> Observers</a></li><li class="chapter-item expanded "><a href="guides/python-step.html"><strong aria-hidden="true">3.9.</strong> Python Step</a></li></ol></li><li class="chapter-item expanded "><a href="reference/index.html"><strong aria-hidden="true">4.</strong> Reference</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="reference/client.html"><strong aria-hidden="true">4.1.</strong> Client API</a></li><li class="chapter-item expanded "><a href="reference/workflow.html"><strong aria-hidden="true">4.2.</strong> Workflow API</a></li><li class="chapter-item expanded "><a href="reference/dependencies.html"><strong aria-hidden="true">4.3.</strong> Dependencies</a></li><li class="chapter-item expanded "><a href="reference/configuration.html"><strong aria-hidden="true">4.4.</strong> Configuration</a></li><li class="chapter-item expanded "><a href="reference/troubleshooting.html"><strong aria-hidden="true">4.5.</strong> Troubleshooting</a></li></ol></li><li class="chapter-item expanded "><a href="testing.html"><strong aria-hidden="true">5.</strong> Testing Guide</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
