(function() {
    'use strict';

    const state = {
        tabs: [
            { id: 'tab-1', title: 'Nexora Home', url: 'nexora://home' }
        ],
        activeTabId: 'tab-1',
        shieldCount: 0,
        isBookmarked: false,
        showBookmarksBar: true,
        zoomLevel: 100,
        selectedIndex: -1,
        suggestions: []
    };

    // DOM Elements
    const tabStrip = document.getElementById('tab-strip');
    const tabStripRow = document.getElementById('tab-strip-row');
    const dragSpacer = document.getElementById('drag-spacer');
    const btnNewTab = document.getElementById('btn-new-tab');
    const btnBack = document.getElementById('btn-back');
    const btnForward = document.getElementById('btn-forward');
    const btnReload = document.getElementById('btn-reload');
    const omniboxForm = document.getElementById('omnibox-form');
    const omniboxInput = document.getElementById('omnibox-input');
    const omniboxDropdown = document.getElementById('omnibox-dropdown');
    const btnStar = document.getElementById('btn-star');
    const btnSiteInfo = document.getElementById('btn-site-info');
    const btnShield = document.getElementById('btn-shield');
    const shieldCount = document.getElementById('shield-count');
    const btnMoreMenu = document.getElementById('btn-more-menu');
    const moreMenuDropdown = document.getElementById('more-menu-dropdown');
    const securityPopover = document.getElementById('security-popover');
    const popoverShieldVal = document.getElementById('popover-shield-val');
    const bookmarksBarRow = document.getElementById('bookmarks-bar-row');
    const bookmarksBarList = document.getElementById('bookmarks-bar-list');

    // Caption Controls
    const btnWinMin = document.getElementById('btn-win-min');
    const btnWinMax = document.getElementById('btn-win-max');
    const btnWinClose = document.getElementById('btn-win-close');

    // Menu Action Items
    const menuNewTab = document.getElementById('menu-new-tab');
    const menuHistory = document.getElementById('menu-history');
    const menuBookmarks = document.getElementById('menu-bookmarks');
    const menuDownloads = document.getElementById('menu-downloads');
    const menuSettings = document.getElementById('menu-settings');
    const menuExit = document.getElementById('menu-exit');
    const zoomMinus = document.getElementById('zoom-minus');
    const zoomPlus = document.getElementById('zoom-plus');
    const zoomLevelEl = document.getElementById('zoom-level');

    function sendIpc(msg) {
        if (window.ipc) {
            window.ipc.postMessage(JSON.stringify(msg));
        } else {
            console.log('[Nexora IPC]:', msg);
        }
    }

    // --- Tab Management ---

    function renderTabs() {
        tabStrip.innerHTML = '';
        state.tabs.forEach(tab => {
            const isActive = tab.id === state.activeTabId;
            const tabEl = document.createElement('div');
            tabEl.className = `tab-item ${isActive ? 'active' : ''}`;
            tabEl.setAttribute('role', 'tab');
            tabEl.setAttribute('aria-selected', isActive ? 'true' : 'false');
            tabEl.dataset.id = tab.id;

            tabEl.innerHTML = `
                <svg class="tab-favicon" viewBox="0 0 16 16" fill="currentColor" aria-hidden="true">
                    <path d="M8 0a8 8 0 1 0 0 16A8 8 0 0 0 8 0ZM1.5 8a6.5 6.5 0 1 1 13 0 6.5 6.5 0 0 1-13 0Zm7-5.46v10.92a6.51 6.51 0 0 1-3.25-1.46 6.51 6.51 0 0 1 0-8 6.51 6.51 0 0 1 3.25-1.46Z"/>
                </svg>
                <span class="tab-title" title="${escapeHtml(tab.title)}">${escapeHtml(tab.title)}</span>
                <button class="tab-close-btn" title="Close Tab" aria-label="Close tab">&times;</button>
            `;

            tabEl.addEventListener('click', (e) => {
                if (e.target.classList.contains('tab-close-btn')) {
                    e.stopPropagation();
                    closeTab(tab.id);
                } else {
                    switchTab(tab.id);
                }
            });

            tabStrip.appendChild(tabEl);
        });
    }

    function switchTab(id) {
        state.activeTabId = id;
        const activeTab = state.tabs.find(t => t.id === id);
        if (activeTab) {
            omniboxInput.value = (activeTab.url === 'nexora://home' || activeTab.url === '') ? '' : activeTab.url;
            hideOmniboxDropdown();
            closeOverlays();
            sendIpc({ type: 'switch_tab', id: id });
        }
        renderTabs();
    }

    function createNewTab() {
        const newId = 'tab-' + Date.now();
        const newTab = { id: newId, title: 'Nexora Home', url: 'nexora://home' };
        state.tabs.push(newTab);
        state.activeTabId = newId;
        omniboxInput.value = '';
        renderTabs();
        hideOmniboxDropdown();
        closeOverlays();
        sendIpc({ type: 'new_tab', id: newId });
    }

    function closeTab(id) {
        if (state.tabs.length <= 1) {
            state.tabs[0].url = 'nexora://home';
            state.tabs[0].title = 'Nexora Home';
            switchTab(state.tabs[0].id);
            sendIpc({ type: 'navigate', url: 'nexora://home' });
            return;
        }

        const index = state.tabs.findIndex(t => t.id === id);
        if (index === -1) return;

        state.tabs.splice(index, 1);
        if (state.activeTabId === id) {
            const nextIndex = Math.max(0, index - 1);
            state.activeTabId = state.tabs[nextIndex].id;
            switchTab(state.activeTabId);
        } else {
            renderTabs();
        }
        sendIpc({ type: 'close_tab', id: id });
    }

    // --- Navigation ---

    function navigate(rawInput) {
        const input = rawInput.trim();
        if (!input) return;

        let targetUrl = input;
        if (input.startsWith('nexora://')) {
            targetUrl = input;
        } else if (input === 'about:blank' || input === 'home') {
            targetUrl = 'nexora://home';
        } else if (input.startsWith('http://') || input.startsWith('https://')) {
            targetUrl = input;
        } else if (input.startsWith('localhost') || input.startsWith('127.0.0.1')) {
            targetUrl = 'http://' + input;
        } else if (input.includes('.') && !input.includes(' ')) {
            targetUrl = 'https://' + input;
        } else {
            // Google clean web search without bot/CAPTCHA walls
            targetUrl = 'https://www.google.com/search?q=' + encodeURIComponent(input);
        }

        const active = state.tabs.find(t => t.id === state.activeTabId);
        if (active) {
            active.url = targetUrl;
            omniboxInput.value = (targetUrl === 'nexora://home') ? '' : targetUrl;
            renderTabs();
            hideOmniboxDropdown();
            closeOverlays();
            sendIpc({ type: 'navigate', url: targetUrl });
        }
    }

    function escapeHtml(str) {
        if (!str) return '';
        return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
    }

    // --- Omnibox Dropdown Autocomplete ---

    function hideOmniboxDropdown() {
        omniboxDropdown.classList.remove('show');
        omniboxDropdown.innerHTML = '';
        state.selectedIndex = -1;
        state.suggestions = [];
    }

    function renderOmniboxSuggestions(suggestions) {
        if (!suggestions || suggestions.length === 0) {
            hideOmniboxDropdown();
            return;
        }

        state.suggestions = suggestions;
        state.selectedIndex = -1;
        omniboxDropdown.innerHTML = '';

        suggestions.forEach((item, index) => {
            const el = document.createElement('div');
            el.className = 'suggestion-item';
            el.dataset.index = index;

            let iconSvg = '<svg class="suggestion-icon" viewBox="0 0 16 16" fill="currentColor"><path d="M8 0a8 8 0 1 0 0 16A8 8 0 0 0 8 0ZM1.5 8a6.5 6.5 0 1 1 13 0 6.5 6.5 0 0 1-13 0Zm7-5.46v10.92a6.51 6.51 0 0 1-3.25-1.46 6.51 6.51 0 0 1 0-8 6.51 6.51 0 0 1 3.25-1.46Z"/></svg>';
            if (item.match_type === 'bookmark') {
                iconSvg = '<svg class="suggestion-icon" viewBox="0 0 16 16" fill="#eab308"><path d="m8 1.75 2.122 4.298 4.745.69-3.434 3.346.81 4.726L8 12.585l-4.243 2.225.81-4.726L1.133 6.738l4.745-.69L8 1.75Z"/></svg>';
            } else if (item.match_type === 'history') {
                iconSvg = '<svg class="suggestion-icon" viewBox="0 0 16 16" fill="currentColor"><path d="M8 0a8 8 0 1 0 0 16A8 8 0 0 0 8 0Zm0 1.5a6.5 6.5 0 1 1 0 13 6.5 6.5 0 0 1 0-13ZM7.25 4v5.25h4v-1.5H8.75V4h-1.5Z"/></svg>';
            }

            el.innerHTML = `
                ${iconSvg}
                <div class="suggestion-text">
                    <span class="suggestion-title">${escapeHtml(item.title || item.url)}</span>
                    <span class="suggestion-url">${escapeHtml(item.url)}</span>
                </div>
            `;

            el.addEventListener('mousedown', (e) => {
                e.preventDefault();
                navigate(item.url);
            });

            omniboxDropdown.appendChild(el);
        });

        omniboxDropdown.classList.add('show');
    }

    // --- Bookmarks Bar ---

    function renderBookmarksBar(bookmarks) {
        bookmarksBarList.innerHTML = '';
        if (!bookmarks || bookmarks.length === 0) return;

        bookmarks.forEach(bm => {
            const chip = document.createElement('a');
            chip.className = 'bookmark-chip';
            chip.href = '#';
            chip.title = `${bm.title} (${bm.url})`;
            chip.innerHTML = `
                <svg class="bookmark-chip-icon" viewBox="0 0 16 16" fill="currentColor">
                    <path d="m8 1.75 2.122 4.298 4.745.69-3.434 3.346.81 4.726L8 12.585l-4.243 2.225.81-4.726L1.133 6.738l4.745-.69L8 1.75Z"/>
                </svg>
                <span>${escapeHtml(bm.title || bm.url)}</span>
            `;

            chip.addEventListener('click', (e) => {
                e.preventDefault();
                navigate(bm.url);
            });

            bookmarksBarList.appendChild(chip);
        });
    }

    // --- Overlays (More Menu & Popover) ---

    function closeOverlays() {
        moreMenuDropdown.classList.remove('show');
        securityPopover.classList.remove('show');
    }

    function toggleMoreMenu() {
        const isOpen = moreMenuDropdown.classList.contains('show');
        closeOverlays();
        if (!isOpen) {
            moreMenuDropdown.classList.add('show');
        }
    }

    function toggleSecurityPopover() {
        const isOpen = securityPopover.classList.contains('show');
        closeOverlays();
        if (!isOpen) {
            securityPopover.classList.add('show');
        }
    }

    // --- Event Listeners ---

    omniboxForm.addEventListener('submit', (e) => {
        e.preventDefault();
        if (state.selectedIndex >= 0 && state.suggestions[state.selectedIndex]) {
            navigate(state.suggestions[state.selectedIndex].url);
        } else {
            navigate(omniboxInput.value);
        }
        omniboxInput.blur();
    });

    omniboxInput.addEventListener('input', (e) => {
        const val = e.target.value.trim();
        if (val.length === 0 || val.startsWith('nexora://')) {
            hideOmniboxDropdown();
        } else {
            sendIpc({ type: 'omnibox_query', query: val });
        }
    });

    omniboxInput.addEventListener('keydown', (e) => {
        if (!omniboxDropdown.classList.contains('show')) return;

        const items = omniboxDropdown.querySelectorAll('.suggestion-item');
        if (e.key === 'ArrowDown') {
            e.preventDefault();
            state.selectedIndex = (state.selectedIndex + 1) % items.length;
            updateSelection(items);
        } else if (e.key === 'ArrowUp') {
            e.preventDefault();
            state.selectedIndex = (state.selectedIndex - 1 + items.length) % items.length;
            updateSelection(items);
        } else if (e.key === 'Escape') {
            hideOmniboxDropdown();
        }
    });

    function updateSelection(items) {
        items.forEach((item, idx) => {
            if (idx === state.selectedIndex) {
                item.classList.add('selected');
                omniboxInput.value = state.suggestions[idx].url;
            } else {
                item.classList.remove('selected');
            }
        });
    }

    btnNewTab.addEventListener('click', createNewTab);
    btnBack.addEventListener('click', () => sendIpc({ type: 'go_back' }));
    btnForward.addEventListener('click', () => sendIpc({ type: 'go_forward' }));
    btnReload.addEventListener('click', () => sendIpc({ type: 'reload' }));

    btnStar.addEventListener('click', () => {
        const active = state.tabs.find(t => t.id === state.activeTabId);
        if (active && active.url && !active.url.startsWith('nexora://')) {
            sendIpc({ type: 'toggle_bookmark', url: active.url, title: active.title });
        }
    });

    btnMoreMenu.addEventListener('click', (e) => {
        e.stopPropagation();
        toggleMoreMenu();
    });

    btnSiteInfo.addEventListener('click', (e) => {
        e.stopPropagation();
        toggleSecurityPopover();
    });

    btnShield.addEventListener('click', (e) => {
        e.stopPropagation();
        toggleSecurityPopover();
    });

    // Close overlays when clicking outside
    document.addEventListener('click', (e) => {
        if (!e.target.closest('#more-menu-dropdown') && !e.target.closest('#btn-more-menu') &&
            !e.target.closest('#security-popover') && !e.target.closest('#btn-site-info') && !e.target.closest('#btn-shield')) {
            closeOverlays();
        }
        if (!e.target.closest('#omnibox-shell') && !e.target.closest('#omnibox-dropdown')) {
            hideOmniboxDropdown();
        }
    });

    // Menu Actions
    menuNewTab.addEventListener('click', () => {
        closeOverlays();
        createNewTab();
    });
    menuHistory.addEventListener('click', () => {
        closeOverlays();
        navigate('nexora://history');
    });
    menuBookmarks.addEventListener('click', () => {
        closeOverlays();
        navigate('nexora://bookmarks');
    });
    menuDownloads.addEventListener('click', () => {
        closeOverlays();
        navigate('nexora://downloads');
    });
    menuSettings.addEventListener('click', () => {
        closeOverlays();
        navigate('nexora://settings');
    });
    menuExit.addEventListener('click', () => {
        sendIpc({ type: 'close_window' });
    });

    zoomMinus.addEventListener('click', (e) => {
        e.stopPropagation();
        if (state.zoomLevel > 50) {
            state.zoomLevel -= 10;
            zoomLevelEl.textContent = state.zoomLevel + '%';
            sendIpc({ type: 'set_zoom', level: state.zoomLevel });
        }
    });

    zoomPlus.addEventListener('click', (e) => {
        e.stopPropagation();
        if (state.zoomLevel < 200) {
            state.zoomLevel += 10;
            zoomLevelEl.textContent = state.zoomLevel + '%';
            sendIpc({ type: 'set_zoom', level: state.zoomLevel });
        }
    });

    // Window Caption Controls
    if (btnWinMin) btnWinMin.addEventListener('click', () => sendIpc({ type: 'minimize_window' }));
    if (btnWinMax) btnWinMax.addEventListener('click', () => sendIpc({ type: 'toggle_maximize_window' }));
    if (btnWinClose) btnWinClose.addEventListener('click', () => sendIpc({ type: 'close_window' }));

    // Window Dragging
    function setupDrag(element) {
        if (!element) return;
        element.addEventListener('mousedown', (e) => {
            if (e.button === 0 && e.target === element) {
                sendIpc({ type: 'drag_window' });
            }
        });
        element.addEventListener('dblclick', (e) => {
            if (e.target === element) {
                sendIpc({ type: 'toggle_maximize_window' });
            }
        });
    }

    setupDrag(dragSpacer);
    setupDrag(tabStripRow);
    setupDrag(tabStrip);

    // Keyboard Shortcuts
    document.addEventListener('keydown', (e) => {
        if (e.ctrlKey) {
            if (e.key === 't' || e.key === 'T') {
                e.preventDefault();
                createNewTab();
            } else if (e.key === 'w' || e.key === 'W') {
                e.preventDefault();
                closeTab(state.activeTabId);
            } else if (e.key === 'l' || e.key === 'L') {
                e.preventDefault();
                omniboxInput.focus();
                omniboxInput.select();
            } else if (e.key === 'r' || e.key === 'R') {
                e.preventDefault();
                sendIpc({ type: 'reload' });
            } else if (e.key === 'h' || e.key === 'H') {
                e.preventDefault();
                navigate('nexora://history');
            } else if (e.key === 'j' || e.key === 'J') {
                e.preventDefault();
                navigate('nexora://downloads');
            } else if (e.key === 'd' || e.key === 'D') {
                e.preventDefault();
                const active = state.tabs.find(t => t.id === state.activeTabId);
                if (active && active.url && !active.url.startsWith('nexora://')) {
                    sendIpc({ type: 'toggle_bookmark', url: active.url, title: active.title });
                }
            } else if (e.shiftKey && (e.key === 'B' || e.key === 'b')) {
                e.preventDefault();
                state.showBookmarksBar = !state.showBookmarksBar;
                bookmarksBarRow.classList.toggle('collapsed', !state.showBookmarksBar);
                sendIpc({ type: 'toggle_bookmarks_bar', show: state.showBookmarksBar });
            } else if (e.shiftKey && (e.key === 'O' || e.key === 'o')) {
                e.preventDefault();
                navigate('nexora://bookmarks');
            }
        }
    });

    // --- Global Hooks Called from Rust ---

    window.__nexora_update_url = function(url) {
        const active = state.tabs.find(t => t.id === state.activeTabId);
        if (active) {
            active.url = url;
            omniboxInput.value = (url === 'nexora://home' || url === 'about:blank') ? '' : url;
        }
    };

    window.__nexora_update_tab_title = function(tabId, title) {
        const tab = state.tabs.find(t => t.id === tabId);
        if (tab && title && title !== 'Untitled' && title !== 'about:blank') {
            tab.title = title;
            renderTabs();
        }
    };

    window.__nexora_update_title = function(title) {
        window.__nexora_update_tab_title(state.activeTabId, title);
    };

    window.__nexora_update_shield = function(count) {
        state.shieldCount = count;
        shieldCount.textContent = count;
        popoverShieldVal.textContent = `Active (${count} blocked)`;
    };

    window.__nexora_set_bookmarked = function(isBookmarked) {
        state.isBookmarked = isBookmarked;
        btnStar.classList.toggle('bookmarked', isBookmarked);
    };

    window.__nexora_render_bookmarks_bar = function(bookmarks) {
        renderBookmarksBar(bookmarks);
    };

    window.__nexora_render_omnibox_suggestions = function(suggestions) {
        renderOmniboxSuggestions(suggestions);
    };

    // Initial setup
    renderTabs();
    sendIpc({ type: 'get_bookmarks_bar' });
})();
