(function() {
    'use strict';

    const state = {
        tabs: [
            { id: 'tab-1', title: 'Nexora Home', url: 'nexora://home' }
        ],
        activeTabId: 'tab-1',
        shieldCount: 0
    };

    const tabStrip = document.getElementById('tab-strip');
    const btnNewTab = document.getElementById('btn-new-tab');
    const btnBack = document.getElementById('btn-back');
    const btnForward = document.getElementById('btn-forward');
    const btnReload = document.getElementById('btn-reload');
    const omniboxForm = document.getElementById('omnibox-form');
    const omniboxInput = document.getElementById('omnibox-input');
    const shieldCount = document.getElementById('shield-count');

    function sendIpc(msg) {
        if (window.ipc) {
            window.ipc.postMessage(JSON.stringify(msg));
        } else {
            console.log('[Nexora IPC]:', msg);
        }
    }

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
                <span class="tab-title">${escapeHtml(tab.title)}</span>
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
            sendIpc({ type: 'switch_tab', id: id, url: activeTab.url });
        }
        renderTabs();
    }

    function createNewTab() {
        const newId = 'tab-' + Date.now();
        const newTab = { id: newId, title: 'Nexora Home', url: 'nexora://home' };
        state.tabs.push(newTab);
        switchTab(newId);
        sendIpc({ type: 'new_tab', id: newId });
    }

    function closeTab(id) {
        if (state.tabs.length <= 1) {
            state.tabs[0].url = 'nexora://home';
            state.tabs[0].title = 'Nexora Home';
            switchTab(state.tabs[0].id);
            return;
        }

        const index = state.tabs.findIndex(t => t.id === id);
        if (index === -1) return;

        state.tabs.splice(index, 1);
        if (state.activeTabId === id) {
            const nextIndex = Math.max(0, index - 1);
            switchTab(state.tabs[nextIndex].id);
        } else {
            renderTabs();
        }
        sendIpc({ type: 'close_tab', id: id });
    }

    function navigate(rawInput) {
        const input = rawInput.trim();
        if (!input) return;

        let targetUrl = input;
        if (input === 'nexora://home' || input === 'about:blank') {
            targetUrl = 'nexora://home';
        } else if (input.startsWith('http://') || input.startsWith('https://')) {
            targetUrl = input;
        } else if (input.startsWith('localhost') || input.startsWith('127.0.0.1')) {
            targetUrl = 'http://' + input;
        } else if (input.includes('.') && !input.includes(' ')) {
            // Standard web address like supabase.com, github.com, etc.
            targetUrl = 'https://' + input;
        } else {
            // Clean web search without bot challenges
            targetUrl = 'https://www.google.com/search?q=' + encodeURIComponent(input);
        }

        const active = state.tabs.find(t => t.id === state.activeTabId);
        if (active) {
            active.url = targetUrl;
            omniboxInput.value = targetUrl;
            renderTabs();
            sendIpc({ type: 'navigate', url: targetUrl });
        }
    }

    function escapeHtml(str) {
        return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
    }

    // Event Handlers
    omniboxForm.addEventListener('submit', (e) => {
        e.preventDefault();
        navigate(omniboxInput.value);
        omniboxInput.blur();
    });

    btnNewTab.addEventListener('click', createNewTab);
    btnBack.addEventListener('click', () => sendIpc({ type: 'go_back' }));
    btnForward.addEventListener('click', () => sendIpc({ type: 'go_forward' }));
    btnReload.addEventListener('click', () => sendIpc({ type: 'reload' }));

    // Global Hooks called from Rust
    window.__nexora_update_url = function(url) {
        const active = state.tabs.find(t => t.id === state.activeTabId);
        if (active) {
            active.url = url;
            omniboxInput.value = (url === 'nexora://home' || url === 'about:blank') ? '' : url;
        }
    };

    window.__nexora_update_title = function(title) {
        const active = state.tabs.find(t => t.id === state.activeTabId);
        if (active && title && title !== 'Untitled' && title !== 'about:blank') {
            active.title = title;
            renderTabs();
        }
    };

    window.__nexora_update_shield = function(count) {
        state.shieldCount = count;
        shieldCount.textContent = count;
    };

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
            }
        }
    });

    renderTabs();
})();
