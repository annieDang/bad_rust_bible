(function () {
    const cfg = window.SITE_CONFIG || {};
    const root = cfg.root || '';
    const crumbs = cfg.breadcrumbs || [];
    const isHome = crumbs.length === 0;

    // ── Nav ──────────────────────────────────────────────
    const navEl = document.getElementById('site-nav');
    if (navEl) {
        const titleHtml = isHome
            ? '<span class="site-title">The Bad Rust Bible</span>'
            : `<a href="${root}index.html" class="site-title">The Bad Rust Bible</a>`;

        const crumbHtml = crumbs.map(function (c) {
            const sep = '<span class="sep">/</span>';
            return c.href
                ? sep + `<a href="${c.href}">${c.label}</a>`
                : sep + `<span class="crumb-current">${c.label}</span>`;
        }).join('');

        const backHtml = !isHome
            ? `<a href="${root}index.html" class="nav-back">← All Chapters</a>`
            : '';

        navEl.innerHTML =
            `<div class="nav-left">${titleHtml}<span class="breadcrumb">${crumbHtml}</span></div>` +
            backHtml;
    }

    // ── Footer ───────────────────────────────────────────
    const footerEl = document.getElementById('site-footer');
    if (footerEl) {
        const backHtml = !isHome
            ? `<a href="${root}index.html">← Back to all chapters</a>`
            : '';

        footerEl.innerHTML =
            `<span>© Annie Dang — The Bad Rust Bible</span>` +
            backHtml;
    }
}());
