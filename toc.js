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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="introduction.html">Introduction</a></li><li class="chapter-item expanded affix "><li class="part-title">Getting Started</li><li class="chapter-item expanded "><a href="install/index.html"><strong aria-hidden="true">1.</strong> Installation</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="install/android-windows.html"><strong aria-hidden="true">1.1.</strong> Android on Windows</a></li><li class="chapter-item expanded "><a href="install/android-linux.html"><strong aria-hidden="true">1.2.</strong> Android on Linux</a></li><li class="chapter-item expanded "><a href="install/android-macos.html"><strong aria-hidden="true">1.3.</strong> Android on macOS</a></li><li class="chapter-item expanded "><a href="install/docker.html"><strong aria-hidden="true">1.4.</strong> Android with Docker</a></li><li class="chapter-item expanded "><a href="install/ios-macos.html"><strong aria-hidden="true">1.5.</strong> iOS on macOS</a></li><li class="chapter-item expanded "><a href="install/set-up-android-device.html"><strong aria-hidden="true">1.6.</strong> Set up device</a></li></ol></li><li class="chapter-item expanded "><li class="part-title">Components</li><li class="chapter-item expanded "><a href="crossbundle/index.html"><strong aria-hidden="true">2.</strong> Crossbundle</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="crossbundle/command-install.html"><strong aria-hidden="true">2.1.</strong> Install Command</a></li><li class="chapter-item expanded "><a href="crossbundle/command-doctor.html"><strong aria-hidden="true">2.2.</strong> Doctor Command</a></li><li class="chapter-item expanded "><a href="crossbundle/command-build.html"><strong aria-hidden="true">2.3.</strong> Build Command</a></li><li class="chapter-item expanded "><a href="crossbundle/command-run.html"><strong aria-hidden="true">2.4.</strong> Run Command</a></li><li class="chapter-item expanded "><a href="crossbundle/command-new.html"><strong aria-hidden="true">2.5.</strong> New Command</a></li><li class="chapter-item expanded "><a href="crossbundle/command-update.html"><strong aria-hidden="true">2.6.</strong> Update command</a></li></ol></li><li class="chapter-item expanded "><a href="crossbow/index.html"><strong aria-hidden="true">3.</strong> Crossbow</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="crossbow/configuration.html"><strong aria-hidden="true">3.1.</strong> Configuration</a></li><li class="chapter-item expanded "><a href="crossbow/permissions.html"><strong aria-hidden="true">3.2.</strong> Permissions</a></li><li class="chapter-item expanded "><a href="crossbow/android-plugins.html"><strong aria-hidden="true">3.3.</strong> Android Plugins</a></li></ol></li><li class="chapter-item expanded "><li class="part-title">Tutorials</li><li class="chapter-item expanded "><a href="tutorials/index.html"><strong aria-hidden="true">4.</strong> Examples</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="tutorials/hello-world.html"><strong aria-hidden="true">4.1.</strong> Hello World</a></li><li class="chapter-item expanded "><a href="tutorials/in-app-updates.html"><strong aria-hidden="true">4.2.</strong> In-app updates tutorial</a></li><li class="chapter-item expanded "><a href="tutorials/play-billing.html"><strong aria-hidden="true">4.3.</strong> Google Play Billing tutorial</a></li><li class="chapter-item expanded "><a href="tutorials/fastlane.html"><strong aria-hidden="true">4.4.</strong> Fastlane</a></li></ol></li><li class="chapter-item expanded "><li class="part-title">Other</li><li class="chapter-item expanded "><a href="contributing/index.html"><strong aria-hidden="true">5.</strong> Contributing</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="contributing/contribute-code.html"><strong aria-hidden="true">5.1.</strong> Contribute Code</a></li><li class="chapter-item expanded "><a href="contributing/contribute-docs.html"><strong aria-hidden="true">5.2.</strong> Contribute Docs</a></li><li class="chapter-item expanded "><a href="contributing/testing-guide.html"><strong aria-hidden="true">5.3.</strong> Testing Guide</a></li><li class="chapter-item expanded "><a href="contributing/contributors.html"><strong aria-hidden="true">5.4.</strong> Contributors</a></li></ol></li><li class="chapter-item expanded "><a href="roadmap.html"><strong aria-hidden="true">6.</strong> Roadmap</a></li><li class="chapter-item expanded "><a href="special-thanks.html"><strong aria-hidden="true">7.</strong> Special Thanks</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
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
