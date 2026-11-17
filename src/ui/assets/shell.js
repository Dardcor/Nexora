(function() {
    'use strict';

    // State
    const state = {
        tabs: [
            {
                id: 'tab-1',
                title: 'New Tab',
                url: 'newtab.html',
                space: 'personal',
                isHibernated: false,
                isReader: false
            }
        ],
        activeTabId: 'tab-1',
        activeSpace: 'personal',
        trackersBlocked: 0,
        dataSavedKB: 0,
        isShieldModalOpen: false,
        isSidebarCollapsed: false,
    };

    // Elements
    const omnibox = document.getElementById('omnibox');
    const tabList = document.getElementById('tab-list');
    const mainFrame = document.getElementById('main-frame');
    const shieldToggle = document.getElementById('shield-toggle');
    const shieldModal = document.getElementById('shield-modal');
    const shieldCount = document.getElementById('shield-count');
    const modalTrackers = document.getElementById('modal-trackers');
    const modalDataSaved = document.getElementById('modal-data-saved');
    const sidebar = document.getElementById('sidebar');
    const btnToggleSidebar = document.getElementById('btn-toggle-sidebar');
    const btnNewTab = document.getElementById('btn-new-tab');
    const btnBack = document.getElementById('btn-back');
    const btnForward = document.getElementById('btn-forward');
    const btnReload = document.getElementById('btn-reload');
    const btnHome = document.getElementById('btn-home');
    const btnReader = document.getElementById('btn-reader');
    const hibernatedCountEl = document.getElementById('hibernated-count');

    // IPC Communication with Rust Core
    function sendIpc(message) {
        if (window.ipc) {
            window.ipc.postMessage(JSON.stringify(message));
        } else {
            console.log('[Nexora IPC]:', message);
        }
    }

    // Tab Management
    function renderTabs() {
        tabList.innerHTML = '';
        const visibleTabs = state.tabs.filter(t => t.space === state.activeSpace);

        let hibernatedTotal = 0;

        visibleTabs.forEach(tab => {
            if (tab.isHibernated) hibernatedTotal++;

            const tabEl = document.createElement('div');
            tabEl.className = `tab-item ${tab.id === state.activeTabId ? 'active' : ''}`;
            tabEl.innerHTML = `
                <span class="tab-icon">${tab.url === 'newtab.html' ? '⚡' : '🌐'}</span>
                <span class="tab-title" title="${tab.title}">${tab.title}</span>
                ${tab.isHibernated ? '<span class="tab-sleep-badge" title="Tab Hibernated (RAM Saved)">💤</span>' : ''}
                <button class="tab-close-btn" title="Close Tab">&times;</button>
            `;

            tabEl.addEventListener('click', (e) => {
                if (e.target.classList.contains('tab-close-btn')) {
                    e.stopPropagation();
                    closeTab(tab.id);
                } else {
                    switchTab(tab.id);
                }
            });

            tabList.appendChild(tabEl);
        });

        hibernatedCountEl.textContent = `${hibernatedTotal} tabs`;
    }

    function switchTab(id) {
        const tab = state.tabs.find(t => t.id === id);
        if (!tab) return;

        state.activeTabId = id;
        if (tab.isHibernated) {
            tab.isHibernated = false; // Hot wake
        }

        omnibox.value = tab.url === 'newtab.html' ? '' : tab.url;
        mainFrame.src = tab.url;

        renderTabs();
        sendIpc({ type: 'switch_tab', id: tab.id });
    }

    function createNewTab(url = 'newtab.html', title = 'New Tab') {
        const newTab = {
            id: 'tab-' + Date.now(),
            title: title,
            url: url,
            space: state.activeSpace,
            isHibernated: false,
            isReader: false
        };

        state.tabs.push(newTab);
        switchTab(newTab.id);
        sendIpc({ type: 'new_tab', tab: newTab });
    }

    function closeTab(id) {
        if (state.tabs.length <= 1) {
            // Keep at least one tab
            state.tabs[0].url = 'newtab.html';
            state.tabs[0].title = 'New Tab';
            switchTab(state.tabs[0].id);
            return;
        }

        const index = state.tabs.findIndex(t => t.id === id);
        if (index === -1) return;

        state.tabs.splice(index, 1);
        if (state.activeTabId === id) {
            const nextTab = state.tabs[Math.max(0, index - 1)];
            switchTab(nextTab.id);
        } else {
            renderTabs();
        }
        sendIpc({ type: 'close_tab', id: id });
    }

    function navigate(rawInput) {
        const input = rawInput.trim();
        if (!input) return;

        let targetUrl = input;
        let title = input;

        if (input.includes('.') && !input.includes(' ')) {
            if (!input.startsWith('http://') && !input.startsWith('https://')) {
                targetUrl = 'https://' + input;
            }
        } else {
            targetUrl = 'https://duckduckgo.com/?q=' + encodeURIComponent(input);
            title = 'Search: ' + input;
        }

        const activeTab = state.tabs.find(t => t.id === state.activeTabId);
        if (activeTab) {
            activeTab.url = targetUrl;
            activeTab.title = title;
            omnibox.value = targetUrl;
            mainFrame.src = targetUrl;
            renderTabs();
            sendIpc({ type: 'navigate', url: targetUrl, tab_id: activeTab.id });
        }
    }

    // Event Listeners
    omnibox.addEventListener('keydown', (e) => {
        if (e.key === 'Enter') {
            navigate(omnibox.value);
            omnibox.blur();
        }
    });

    btnNewTab.addEventListener('click', () => createNewTab());
    btnHome.addEventListener('click', () => navigate('newtab.html'));
    btnReload.addEventListener('click', () => {
        const activeTab = state.tabs.find(t => t.id === state.activeTabId);
        if (activeTab) mainFrame.src = activeTab.url;
    });
    btnBack.addEventListener('click', () => {
        try { mainFrame.contentWindow.history.back(); } catch(e) {}
    });
    btnForward.addEventListener('click', () => {
        try { mainFrame.contentWindow.history.forward(); } catch(e) {}
    });

    btnToggleSidebar.addEventListener('click', () => {
        state.isSidebarCollapsed = !state.isSidebarCollapsed;
        sidebar.classList.toggle('collapsed', state.isSidebarCollapsed);
    });

    // Shield Modal Toggle
    shieldToggle.addEventListener('click', (e) => {
        e.stopPropagation();
        state.isShieldModalOpen = !state.isShieldModalOpen;
        shieldModal.classList.toggle('open', state.isShieldModalOpen);
    });

    document.addEventListener('click', (e) => {
        if (state.isShieldModalOpen && !shieldModal.contains(e.target) && !shieldToggle.contains(e.target)) {
            state.isShieldModalOpen = false;
            shieldModal.classList.remove('open');
        }
    });

    // Spaces Switcher
    document.querySelectorAll('.space-tab').forEach(btn => {
        btn.addEventListener('click', () => {
            document.querySelectorAll('.space-tab').forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
            state.activeSpace = btn.dataset.space;

            const visible = state.tabs.filter(t => t.space === state.activeSpace);
            if (visible.length === 0) {
                createNewTab('newtab.html', `${btn.querySelector('.space-name').textContent} Tab`);
            } else {
                switchTab(visible[0].id);
            }
        });
    });

    // Reader Mode Toggle
    btnReader.addEventListener('click', () => {
        const activeTab = state.tabs.find(t => t.id === state.activeTabId);
        if (activeTab) {
            sendIpc({ type: 'toggle_reader', tab_id: activeTab.id, url: activeTab.url });
        }
    });

    // Keyboard Shortcuts (Ctrl+T, Ctrl+W, Ctrl+B, Ctrl+L)
    document.addEventListener('keydown', (e) => {
        if (e.ctrlKey) {
            if (e.key === 't' || e.key === 'T') {
                e.preventDefault();
                createNewTab();
            } else if (e.key === 'w' || e.key === 'W') {
                e.preventDefault();
                closeTab(state.activeTabId);
            } else if (e.key === 'b' || e.key === 'B') {
                e.preventDefault();
                btnToggleSidebar.click();
            } else if (e.key === 'l' || e.key === 'L') {
                e.preventDefault();
                omnibox.focus();
                omnibox.select();
            }
        }
    });

    // Listen for incoming messages from Rust Core or iframes
    window.addEventListener('message', (event) => {
        if (!event.data) return;
        try {
            const data = typeof event.data === 'string' ? JSON.parse(event.data) : event.data;
            if (data.type === 'navigate') {
                navigate(data.url);
            } else if (data.type === 'shield_update') {
                state.trackersBlocked = data.count || 0;
                shieldCount.textContent = state.trackersBlocked;
                modalTrackers.textContent = state.trackersBlocked;
                modalDataSaved.textContent = (data.savedKB || 0) + ' KB';
            }
        } catch(e) {}
    });

    // Initial render
    renderTabs();
})();
