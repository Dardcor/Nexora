(function() {
    'use strict';

    const NEXORA_LOGO_DATA_URI = 'data:image/png;base64,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';

    const state = {
        tabs: [
            { id: 'tab-1', title: 'Nexora Home', url: 'nexora://home', favicon: '', pinned: false, groupId: null, createdAt: Date.now() }
        ],
        groups: [],
        activeTabId: 'tab-1',
        shieldCount: 0,
        isBookmarked: false,
        isReaderMode: false,
        isIncognito: false,
        searchEngine: 'google',
        zoomLevel: 100,
        selectedIndex: -1,
        suggestions: [],
        profiles: [],
        activeProfile: null,
        contextTabId: null
    };

    const tabStrip = document.getElementById('tab-strip');
    const tabStripRow = document.getElementById('tab-strip-row');
    const dragSpacer = document.getElementById('drag-spacer');
    const btnNewTab = document.getElementById('btn-new-tab');
    const btnTabMenu = document.getElementById('btn-tab-menu');
    const btnBack = document.getElementById('btn-back');
    const btnForward = document.getElementById('btn-forward');
    const btnReload = document.getElementById('btn-reload');
    const omniboxForm = document.getElementById('omnibox-form');
    const omniboxInput = document.getElementById('omnibox-input');
    const omniboxDropdown = document.getElementById('omnibox-dropdown');
    const btnStar = document.getElementById('btn-star');
    const btnReader = document.getElementById('btn-reader');
    const btnZoomReset = document.getElementById('btn-zoom-reset');
    const zoomBadgeText = document.getElementById('zoom-badge-text');
    const btnSiteInfo = document.getElementById('btn-site-info');
    const btnExtensions = document.getElementById('btn-extensions');
    const extensionsDropdown = document.getElementById('extensions-dropdown');
    const extShieldItem = document.getElementById('ext-shield-item');
    const extReaderItem = document.getElementById('ext-reader-item');
    const btnManageExtensions = document.getElementById('btn-manage-extensions');
    const shieldExtStatus = document.getElementById('shield-ext-status');
    const btnDownloads = document.getElementById('btn-downloads');
    const shieldCount = document.getElementById('shield-count');
    const btnProfile = document.getElementById('btn-profile');
    const profilePopup = document.getElementById('profile-popup');
    const toolbarAvatarCircle = document.getElementById('toolbar-avatar-circle');
    const toolbarAvatarSvg = document.getElementById('toolbar-avatar-svg');
    const toolbarAvatarInitial = document.getElementById('toolbar-avatar-initial');
    const popupAvatarCircle = document.getElementById('popup-avatar-circle');
    const popupAvatarSvg = document.getElementById('popup-avatar-svg');
    const popupAvatarInitial = document.getElementById('popup-avatar-initial');
    const popupProfileBadge = document.getElementById('popup-profile-badge');
    const popupProfileName = document.getElementById('popup-profile-name');
    const popupProfileEmail = document.getElementById('popup-profile-email');
    const profBtnSignin = document.getElementById('prof-btn-signin');
    const profileOtherSection = document.getElementById('profile-other-section');
    const profileOtherList = document.getElementById('profile-other-list');
    const profBtnPasswords = document.getElementById('prof-btn-passwords');
    const profBtnGoogle = document.getElementById('prof-btn-google');
    const profBtnCustomize = document.getElementById('prof-btn-customize');
    const profBtnAccount = document.getElementById('prof-btn-account');
    const profBtnSignout = document.getElementById('prof-btn-signout');
    const profBtnAdd = document.getElementById('prof-btn-add');
    const btnMoreMenu = document.getElementById('btn-more-menu');
    const moreMenuDropdown = document.getElementById('more-menu-dropdown');
    const securityPopover = document.getElementById('security-popover');
    const popoverShieldVal = document.getElementById('popover-shield-val');
    const backdrop = document.getElementById('overlay-backdrop');
    const pageLoadingBar = document.getElementById('page-loading-bar');
    const findBar = document.getElementById('find-bar');
    const findInput = document.getElementById('find-input');
    const findCount = document.getElementById('find-count');
    const findPrev = document.getElementById('find-prev');
    const findNext = document.getElementById('find-next');
    const findClose = document.getElementById('find-close');
    const tabSearchDropdown = document.getElementById('tab-search-dropdown');
    const tabSearchInput = document.getElementById('tab-search-input');
    const tabSearchList = document.getElementById('tab-search-list');
    const tabContextMenu = document.getElementById('tab-context-menu');

    const btnWinMin = document.getElementById('btn-win-min');
    const btnWinMax = document.getElementById('btn-win-max');
    const btnWinClose = document.getElementById('btn-win-close');

    const menuNewTab = document.getElementById('menu-new-tab');
    const menuHistory = document.getElementById('menu-history');
    const menuBookmarks = document.getElementById('menu-bookmarks');
    const menuDownloads = document.getElementById('menu-downloads');
    const menuSettings = document.getElementById('menu-settings');
    const menuExit = document.getElementById('menu-exit');
    const zoomMinus = document.getElementById('zoom-minus');
    const zoomPlus = document.getElementById('zoom-plus');
    const zoomLevelEl = document.getElementById('zoom-level');

    const bookmarksBar = document.getElementById('bookmarks-bar');
    const bookmarkEditDialog = document.getElementById('bookmark-edit-dialog');
    const bmDialogName = document.getElementById('bm-dialog-name');
    const bmDialogFolder = document.getElementById('bm-dialog-folder');
    const btnBmDialogClose = document.getElementById('btn-bm-dialog-close');
    const btnBmDialogDone = document.getElementById('btn-bm-dialog-done');
    const btnBmDialogRemove = document.getElementById('btn-bm-dialog-remove');

    const clearDataDialog = document.getElementById('clear-data-dialog');
    const clearRangeSelect = document.getElementById('clear-range-select');
    const chkClearHistory = document.getElementById('chk-clear-history');
    const chkClearDownloads = document.getElementById('chk-clear-downloads');
    const chkClearCookies = document.getElementById('chk-clear-cookies');
    const btnClearDialogCancel = document.getElementById('btn-clear-dialog-cancel');
    const btnClearDialogClose = document.getElementById('btn-clear-dialog-close');
    const btnClearDialogConfirm = document.getElementById('btn-clear-dialog-confirm');

    const profileModalDialog = document.getElementById('profile-modal-dialog');
    const profModalTitle = document.getElementById('prof-modal-title');
    const btnProfModalClose = document.getElementById('btn-prof-modal-close');
    const profModalAvatar = document.getElementById('prof-modal-avatar');
    const profModalAvatarInitial = document.getElementById('prof-modal-avatar-initial');
    const profModalName = document.getElementById('prof-modal-name');
    const profModalPalette = document.getElementById('prof-modal-palette');
    const btnProfModalCancel = document.getElementById('btn-prof-modal-cancel');
    const btnProfModalSave = document.getElementById('btn-prof-modal-save');

    const tabGroupDialog = document.getElementById('tab-group-dialog');
    const tabGroupModalTitle = document.getElementById('tab-group-modal-title');
    const btnTabGroupClose = document.getElementById('btn-tab-group-close');
    const tabGroupName = document.getElementById('tab-group-name');
    const tabGroupPalette = document.getElementById('tab-group-palette');
    const btnTabGroupDelete = document.getElementById('btn-tab-group-delete');
    const btnTabGroupSave = document.getElementById('btn-tab-group-save');
    const menuItemRemoveGroup = document.getElementById('menu-item-remove-group');

    function sendIpc(msg) {
        if (window.ipc) {
            window.ipc.postMessage(JSON.stringify(msg));
        } else {
            console.log('[Nexora IPC]:', msg);
        }
    }

    function getDomain(url) {
        if (!url) return '';
        try {
            const u = new URL(url);
            return u.hostname;
        } catch (e) {
            return '';
        }
    }

    function escapeHtml(str) {
        if (!str) return '';
        return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
    }

    function expandOverlay(height) {
        if (backdrop) backdrop.classList.add('show');
        sendIpc({ type: 'expand_overlay', height: height || 420 });
    }

    function closeOverlays() {
        const wasOpen = (moreMenuDropdown && moreMenuDropdown.classList.contains('show')) ||
                        (profilePopup && profilePopup.classList.contains('show')) ||
                        (securityPopover && securityPopover.classList.contains('show')) ||
                        (extensionsDropdown && extensionsDropdown.classList.contains('show')) ||
                        (omniboxDropdown && omniboxDropdown.classList.contains('show')) ||
                        (tabSearchDropdown && tabSearchDropdown.classList.contains('show')) ||
                        (tabContextMenu && tabContextMenu.classList.contains('show')) ||
                        (bookmarkEditDialog && bookmarkEditDialog.classList.contains('show')) ||
                        (clearDataDialog && clearDataDialog.classList.contains('show')) ||
                        (profileModalDialog && profileModalDialog.classList.contains('show')) ||
                        (tabGroupDialog && tabGroupDialog.classList.contains('show'));

        if (moreMenuDropdown) moreMenuDropdown.classList.remove('show');
        if (profilePopup) profilePopup.classList.remove('show');
        if (securityPopover) securityPopover.classList.remove('show');
        if (extensionsDropdown) extensionsDropdown.classList.remove('show');
        if (omniboxDropdown) omniboxDropdown.classList.remove('show');
        if (tabSearchDropdown) tabSearchDropdown.classList.remove('show');
        if (tabContextMenu) tabContextMenu.classList.remove('show');
        if (bookmarkEditDialog) bookmarkEditDialog.classList.remove('show');
        if (clearDataDialog) clearDataDialog.classList.remove('show');
        if (profileModalDialog) profileModalDialog.classList.remove('show');
        if (tabGroupDialog) tabGroupDialog.classList.remove('show');
        if (backdrop) backdrop.classList.remove('show');
        state.selectedIndex = -1;

        if (wasOpen) {
            sendIpc({ type: 'collapse_overlay' });
        }
    }

    let syncTabsTimer = null;
    function syncTabsToDatabase() {
        if (state.isIncognito) return;
        if (syncTabsTimer) clearTimeout(syncTabsTimer);
        syncTabsTimer = setTimeout(() => {
            const profileId = state.activeProfile ? state.activeProfile.id : 'default';
            const tabsPayload = state.tabs.map((tab, idx) => ({
                id: tab.id,
                profile_id: profileId,
                url: tab.url || 'nexora://home',
                title: tab.title || 'Nexora Home',
                favicon: tab.favicon || '',
                pinned: !!tab.pinned,
                tab_order: idx,
                group_id: tab.groupId || null,
                is_active: tab.id === state.activeTabId,
                created_at: tab.createdAt || Date.now()
            }));
            sendIpc({ type: 'sync_tabs', tabs: tabsPayload });
        }, 120);
    }

    let activeEditingGroup = null;
    let activeTargetTabId = null;
    let activeGroupColor = '#4285f4';

    function selectGroupColor(color) {
        activeGroupColor = color || '#4285f4';
        if (tabGroupPalette) {
            const swatches = tabGroupPalette.querySelectorAll('.palette-swatch');
            swatches.forEach(swatch => {
                if (swatch.getAttribute('data-color') === activeGroupColor) {
                    swatch.classList.add('active');
                } else {
                    swatch.classList.remove('active');
                }
            });
        }
    }

    function openTabGroupDialog(group, targetTabId, clientX) {
        closeOverlays();
        if (!tabGroupDialog) return;

        if (group) {
            activeEditingGroup = group;
            activeTargetTabId = null;
            if (tabGroupModalTitle) tabGroupModalTitle.textContent = 'Edit tab group';
            if (tabGroupName) tabGroupName.value = group.title || '';
            selectGroupColor(group.color || '#4285f4');
            if (btnTabGroupDelete) btnTabGroupDelete.style.display = 'inline-flex';
        } else {
            activeEditingGroup = null;
            activeTargetTabId = targetTabId;
            if (tabGroupModalTitle) tabGroupModalTitle.textContent = 'Name this group';
            if (tabGroupName) tabGroupName.value = '';
            selectGroupColor('#4285f4');
            if (btnTabGroupDelete) btnTabGroupDelete.style.display = 'none';
        }

        const x = typeof clientX === 'number' ? Math.min(clientX, window.innerWidth - 300) : 80;
        tabGroupDialog.style.left = `${Math.max(10, x)}px`;
        tabGroupDialog.classList.add('show');
        if (backdrop) backdrop.classList.add('show');
        expandOverlay(320);

        setTimeout(() => {
            if (tabGroupName) {
                tabGroupName.focus();
                tabGroupName.select();
            }
        }, 50);
    }

    function saveTabGroupModal() {
        const title = tabGroupName ? (tabGroupName.value.trim() || 'Group') : 'Group';
        if (activeEditingGroup) {
            activeEditingGroup.title = title;
            activeEditingGroup.color = activeGroupColor;
            sendIpc({ type: 'save_tab_group', group: activeEditingGroup });
        } else {
            const newGroupId = 'grp-' + Date.now();
            const newGroup = {
                id: newGroupId,
                title: title,
                color: activeGroupColor,
                collapsed: false,
                created_at: Date.now()
            };
            state.groups.push(newGroup);
            sendIpc({ type: 'save_tab_group', group: newGroup });
            if (activeTargetTabId) {
                const targetTab = state.tabs.find(t => t.id === activeTargetTabId);
                if (targetTab) {
                    targetTab.groupId = newGroupId;
                }
            }
        }
        renderTabs();
        syncTabsToDatabase();
        closeOverlays();
    }

    if (btnTabGroupClose) btnTabGroupClose.addEventListener('click', closeOverlays);
    if (btnTabGroupSave) btnTabGroupSave.addEventListener('click', saveTabGroupModal);
    if (btnTabGroupDelete) {
        btnTabGroupDelete.addEventListener('click', () => {
            if (activeEditingGroup) {
                const gid = activeEditingGroup.id;
                state.groups = state.groups.filter(g => g.id !== gid);
                state.tabs.forEach(t => {
                    if (t.groupId === gid) {
                        t.groupId = null;
                    }
                });
                sendIpc({ type: 'delete_tab_group', id: gid });
                renderTabs();
                syncTabsToDatabase();
            }
            closeOverlays();
        });
    }

    if (tabGroupName) {
        tabGroupName.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                e.preventDefault();
                saveTabGroupModal();
            } else if (e.key === 'Escape') {
                e.preventDefault();
                closeOverlays();
            }
        });
    }

    if (tabGroupPalette) {
        tabGroupPalette.addEventListener('click', (e) => {
            const swatch = e.target.closest('.palette-swatch');
            if (swatch) {
                const color = swatch.getAttribute('data-color');
                if (color) selectGroupColor(color);
            }
        });
    }

    function renderTabs() {
        tabStrip.innerHTML = '';
        const renderedGroupIds = new Set();

        state.tabs.forEach(tab => {
            if (tab.groupId) {
                const group = state.groups.find(g => g.id === tab.groupId) || {
                    id: tab.groupId,
                    title: 'Group',
                    color: '#4285f4',
                    collapsed: false
                };

                if (!renderedGroupIds.has(tab.groupId)) {
                    renderedGroupIds.add(tab.groupId);
                    const badge = document.createElement('div');
                    badge.className = `tab-group-badge ${group.collapsed ? 'collapsed' : ''}`;
                    badge.style.setProperty('--group-color', group.color || '#4285f4');
                    badge.title = `${group.title || 'Group'} (Click to collapse/expand, right-click to edit)`;
                    badge.innerHTML = `
                        <span class="group-dot"></span>
                        <span class="group-title">${escapeHtml(group.title || 'Group')}</span>
                    `;
                    badge.addEventListener('click', (e) => {
                        e.stopPropagation();
                        group.collapsed = !group.collapsed;
                        sendIpc({ type: 'save_tab_group', group: group });
                        renderTabs();
                    });
                    badge.addEventListener('contextmenu', (e) => {
                        e.preventDefault();
                        e.stopPropagation();
                        openTabGroupDialog(group, null, e.clientX);
                    });
                    tabStrip.appendChild(badge);
                }

                if (group.collapsed && tab.id !== state.activeTabId) {
                    return;
                }
            }

            const isActive = tab.id === state.activeTabId;
            const tabEl = document.createElement('div');
            tabEl.className = `tab-item ${isActive ? 'active' : ''} ${tab.pinned ? 'pinned' : ''}`;
            tabEl.setAttribute('role', 'tab');
            tabEl.setAttribute('aria-selected', isActive ? 'true' : 'false');
            tabEl.dataset.id = tab.id;

            if (tab.groupId) {
                const group = state.groups.find(g => g.id === tab.groupId);
                const color = (group && group.color) ? group.color : '#4285f4';
                tabEl.setAttribute('data-group-color', color);
                tabEl.style.setProperty('--tab-group-color', color);
            }

            let faviconHtml = '';
            const rawUrl = tab.url || '';

            if (rawUrl.startsWith('nexora://') || rawUrl === '' || rawUrl === 'about:blank') {
                faviconHtml = `<img class="tab-favicon-img" src="${NEXORA_LOGO_DATA_URI}" alt="Nexora" />`;
            } else {
                const domain = getDomain(rawUrl);
                const s2Url = domain ? `https://www.google.com/s2/favicons?domain=${domain}&sz=32` : '';
                const primarySrc = tab.favicon || s2Url;

                if (primarySrc) {
                    const fallbackAttr = s2Url && primarySrc !== s2Url 
                        ? `this.src='${s2Url}'; this.onerror=function(){this.src='${NEXORA_LOGO_DATA_URI}';};` 
                        : `this.src='${NEXORA_LOGO_DATA_URI}';`;
                    faviconHtml = `<img class="tab-favicon-img" src="${escapeHtml(primarySrc)}" alt="" onerror="${fallbackAttr}" />`;
                } else {
                    faviconHtml = `<img class="tab-favicon-img" src="${NEXORA_LOGO_DATA_URI}" alt="Nexora" />`;
                }
            }

            let audioHtml = '';
            if (tab.isAudible) {
                const icon = tab.isMuted
                    ? '<svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor"><path d="M6.717 3.55A.5.5 0 0 1 7 4v8a.5.5 0 0 1-.812.39L3.825 10.5H1.5A.5.5 0 0 1 1 10V6a.5.5 0 0 1 .5-.5h2.325l2.363-1.89a.5.5 0 0 1 .529-.06zm7.137 2.096a.5.5 0 0 1 0 .708L12.707 7.5l1.147 1.146a.5.5 0 0 1-.708.708L12 8.207l-1.146 1.147a.5.5 0 0 1-.708-.708L11.293 7.5l-1.147-1.146a.5.5 0 0 1 .708-.708L12 6.793l1.146-1.147a.5.5 0 0 1 .708 0z"/></svg>'
                    : '<svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor"><path d="M11.536 14.01A8.473 8.473 0 0 0 14.026 8a8.47 8.47 0 0 0-2.49-6.01l-.708.707A7.476 7.476 0 0 1 13.025 8c0 2.071-.84 3.946-2.197 5.303l.708.707z"/><path d="M10.121 12.596A6.48 6.48 0 0 0 12.025 8a6.48 6.48 0 0 0-1.904-4.596l-.707.707A5.482 5.482 0 0 1 11.025 8a5.482 5.482 0 0 1-1.61 3.89l.706.706z"/><path d="M6.717 3.55A.5.5 0 0 1 7 4v8a.5.5 0 0 1-.812.39L3.825 10.5H1.5A.5.5 0 0 1 1 10V6a.5.5 0 0 1 .5-.5h2.325l2.363-1.89a.5.5 0 0 1 .529-.06z"/></svg>';
                audioHtml = `<button class="tab-audio-btn ${tab.isMuted ? 'muted' : ''}" title="${tab.isMuted ? 'Unmute tab' : 'Mute tab'}">${icon}</button>`;
            }

            if (tab.pinned) {
                tabEl.innerHTML = `${faviconHtml}${audioHtml}`;
            } else {
                tabEl.innerHTML = `
                    ${faviconHtml}
                    <span class="tab-title" title="${escapeHtml(tab.title)}">${escapeHtml(tab.title)}</span>
                    ${audioHtml}
                    <button class="tab-close-btn" title="Close Tab (Ctrl+W)" aria-label="Close tab">&times;</button>
                `;
            }

            tabEl.addEventListener('click', (e) => {
                if (e.target.closest('.tab-audio-btn')) {
                    e.stopPropagation();
                    tab.isMuted = !tab.isMuted;
                    renderTabs();
                    sendIpc({ type: 'toggle_tab_mute', id: tab.id, muted: tab.isMuted });
                } else if (e.target.classList.contains('tab-close-btn')) {
                    e.stopPropagation();
                    closeTab(tab.id);
                } else {
                    switchTab(tab.id);
                }
            });

            tabEl.addEventListener('auxclick', (e) => {
                if (e.button === 1) {
                    e.preventDefault();
                    closeTab(tab.id);
                }
            });

            tabEl.addEventListener('contextmenu', (e) => {
                e.preventDefault();
                state.contextTabId = tab.id;
                if (menuItemRemoveGroup) {
                    menuItemRemoveGroup.style.display = tab.groupId ? 'block' : 'none';
                }
                sendIpc({
                    type: 'tab_context_menu',
                    tab_id: tab.id,
                    screen_x: e.screenX,
                    screen_y: e.screenY
                });
                if (tabContextMenu) {
                    tabContextMenu.style.left = `${Math.min(e.clientX, window.innerWidth - 210)}px`;
                    tabContextMenu.style.top = '36px';
                    tabContextMenu.classList.add('show');
                    expandOverlay(280);
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
            closeOverlays();
            sendIpc({ type: 'switch_tab', id: id });
        }
        renderTabs();
        syncTabsToDatabase();
    }

    function createNewTab(initialUrl, initialTitle) {
        const newId = 'tab-' + Date.now();
        const url = (typeof initialUrl === 'string' && initialUrl) ? initialUrl : 'nexora://home';
        const title = (typeof initialTitle === 'string' && initialTitle) ? initialTitle : 'Nexora Home';
        const newTab = { id: newId, title: title, url: url, favicon: '', pinned: false, groupId: null, createdAt: Date.now() };
        state.tabs.push(newTab);
        state.activeTabId = newId;
        omniboxInput.value = (url === 'nexora://home') ? '' : url;
        renderTabs();
        syncTabsToDatabase();
        closeOverlays();
        sendIpc({ type: 'new_tab', id: newId });
    }

    function closeTab(id) {
        if (state.tabs.length <= 1) {
            state.tabs[0].url = 'nexora://home';
            state.tabs[0].title = 'Nexora Home';
            state.tabs[0].favicon = '';
            state.tabs[0].groupId = null;
            switchTab(state.tabs[0].id);
            syncTabsToDatabase();
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
        syncTabsToDatabase();
        sendIpc({ type: 'close_tab', id: id });
    }

    function navigateOmnibox(inputVal) {
        const val = (inputVal || omniboxInput.value || '').trim();
        if (!val) return;

        closeOverlays();

        let targetUrl = val;
        if (val === 'nexora://home' || val === 'nexora://history' || val === 'nexora://bookmarks' || val === 'nexora://settings' || val === 'nexora://downloads') {
            targetUrl = val;
        } else if (val.startsWith('http://') || val.startsWith('https://')) {
            targetUrl = val;
        } else if (val.includes('.') && !val.includes(' ') && !val.includes('://')) {
            targetUrl = 'https://' + val;
        } else {
            const searchEngines = {
                google: 'https://www.google.com/search?q=',
                duckduckgo: 'https://duckduckgo.com/?q=',
                bing: 'https://www.bing.com/search?q=',
                brave: 'https://search.brave.com/search?q='
            };
            const base = searchEngines[state.searchEngine] || searchEngines.google;
            targetUrl = base + encodeURIComponent(val);
        }

        const active = state.tabs.find(t => t.id === state.activeTabId);
        if (active) {
            active.url = targetUrl;
            omniboxInput.value = (targetUrl === 'nexora://home') ? '' : targetUrl;
            renderTabs();
            syncTabsToDatabase();
        }

        sendIpc({ type: 'navigate', url: targetUrl });
    }

    function renderOmniboxSuggestions(matches) {
        state.suggestions = matches || [];
        state.selectedIndex = -1;

        if (!matches || matches.length === 0) {
            omniboxDropdown.classList.remove('show');
            return;
        }

        omniboxDropdown.innerHTML = '';
        matches.forEach((m, idx) => {
            const item = document.createElement('div');
            item.className = 'omnibox-match-item';
            item.dataset.index = idx;

            let iconSvg = '';
            if (m.match_type === 'bookmark') {
                iconSvg = `<svg class="omnibox-match-icon" viewBox="0 0 16 16" fill="currentColor"><path d="m8 1.75 2.122 4.298 4.745.69-3.434 3.346.81 4.726L8 12.585l-4.243 2.225.81-4.726L1.133 6.738l4.745-.69L8 1.75Z"/></svg>`;
            } else if (m.match_type === 'history') {
                iconSvg = `<svg class="omnibox-match-icon" viewBox="0 0 16 16" fill="currentColor"><path d="M8.515 1.019A7 7 0 0 0 8 1a7 7 0 1 0 7 7h-1.5a5.5 5.5 0 1 1-5.5-5.5v2.5l3.5-3.5L8 0v1.019ZM8 4.5a.75.75 0 0 1 .75.75v2.69l1.72 1.72a.75.75 0 1 1-1.06 1.06l-2-2A.75.75 0 0 1 7.25 8V5.25A.75.75 0 0 1 8 4.5Z"/></svg>`;
            } else {
                iconSvg = `<svg class="omnibox-match-icon" viewBox="0 0 16 16" fill="currentColor"><path d="M11.742 10.344a6.5 6.5 0 1 0-1.397 1.398h-.001c.03.04.062.078.098.115l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1.007 1.007 0 0 0-.115-.1zM12 6.5a5.5 5.5 0 1 1-11 0 5.5 5.5 0 0 1 11 0z"/></svg>`;
            }

            item.innerHTML = `
                ${iconSvg}
                <span class="omnibox-match-text">${escapeHtml(m.title || m.url)}</span>
                <span class="omnibox-match-url">${escapeHtml(m.url)}</span>
            `;

            item.addEventListener('click', () => {
                navigateOmnibox(m.url);
            });

            omniboxDropdown.appendChild(item);
        });

        omniboxDropdown.classList.add('show');
        expandOverlay(280);
    }

    function renderTabSearch(query) {
        const q = (query || '').toLowerCase().trim();
        tabSearchList.innerHTML = '';

        const filtered = state.tabs.filter(t => {
            return !q || (t.title && t.title.toLowerCase().includes(q)) || (t.url && t.url.toLowerCase().includes(q));
        });

        if (filtered.length === 0) {
            tabSearchList.innerHTML = `<div style="padding: 12px 16px; color: var(--text-muted); text-align: center;">No matching tabs found</div>`;
            return;
        }

        filtered.forEach(tab => {
            const item = document.createElement('div');
            item.className = 'tab-search-item';
            const iconSrc = tab.favicon || NEXORA_LOGO_DATA_URI;
            item.innerHTML = `
                <img src="${escapeHtml(iconSrc)}" style="width: 16px; height: 16px; border-radius: 2px;" alt="" />
                <span style="flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">${escapeHtml(tab.title)}</span>
            `;
            item.addEventListener('click', () => {
                switchTab(tab.id);
                closeOverlays();
            });
            tabSearchList.appendChild(item);
        });
    }

    function toggleFindBar() {
        const isShown = findBar.classList.contains('show');
        if (isShown) {
            findBar.classList.remove('show');
            if (findCount) findCount.textContent = '';
            sendIpc({ type: 'stop_find' });
        } else {
            findBar.classList.add('show');
            findInput.focus();
            findInput.select();
        }
    }

    if (findInput) {
        findInput.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                e.preventDefault();
                const q = findInput.value.trim();
                if (q) {
                    sendIpc({ type: 'find_in_page', query: q, forward: !e.shiftKey });
                }
            } else if (e.key === 'Escape') {
                toggleFindBar();
            }
        });
        findInput.addEventListener('input', () => {
            const q = findInput.value.trim();
            if (q) {
                sendIpc({ type: 'find_in_page', query: q, forward: true });
            } else if (findCount) {
                findCount.textContent = '';
            }
        });
    }

    if (findPrev) {
        findPrev.addEventListener('click', () => {
            const q = findInput.value.trim();
            if (q) sendIpc({ type: 'find_in_page', query: q, forward: false });
        });
    }

    if (findNext) {
        findNext.addEventListener('click', () => {
            const q = findInput.value.trim();
            if (q) sendIpc({ type: 'find_in_page', query: q, forward: true });
        });
    }

    if (findClose) {
        findClose.addEventListener('click', toggleFindBar);
    }

    btnNewTab.addEventListener('click', () => createNewTab());
    btnBack.addEventListener('click', () => sendIpc({ type: 'go_back' }));
    btnForward.addEventListener('click', () => sendIpc({ type: 'go_forward' }));
    btnReload.addEventListener('click', () => sendIpc({ type: 'reload' }));

    omniboxForm.addEventListener('submit', (e) => {
        e.preventDefault();
        navigateOmnibox();
    });

    omniboxInput.addEventListener('input', () => {
        const val = omniboxInput.value;
        if (!val.trim()) {
            omniboxDropdown.classList.remove('show');
            return;
        }
        sendIpc({ type: 'omnibox_query', query: val });
    });

    omniboxInput.addEventListener('focus', () => {
        omniboxInput.select();
    });

    omniboxInput.addEventListener('keydown', (e) => {
        const items = omniboxDropdown.querySelectorAll('.omnibox-match-item');
        if (items.length === 0) return;

        if (e.key === 'ArrowDown') {
            e.preventDefault();
            state.selectedIndex = (state.selectedIndex + 1) % items.length;
            updateSelection(items);
        } else if (e.key === 'ArrowUp') {
            e.preventDefault();
            state.selectedIndex = (state.selectedIndex - 1 + items.length) % items.length;
            updateSelection(items);
        } else if (e.key === 'Escape') {
            closeOverlays();
        }
    });

    function updateSelection(items) {
        items.forEach((item, idx) => {
            item.classList.toggle('selected', idx === state.selectedIndex);
        });
        if (state.selectedIndex >= 0 && state.suggestions[state.selectedIndex]) {
            omniboxInput.value = state.suggestions[state.selectedIndex].url;
        }
    }

    function openBookmarkDialog() {
        const active = state.tabs.find(t => t.id === state.activeTabId);
        if (!active) return;
        if (!state.isBookmarked) {
            sendIpc({
                type: 'toggle_bookmark',
                url: active.url,
                title: active.title
            });
            window.__nexora_set_bookmarked(true);
        }
        closeOverlays();
        if (bmDialogName) bmDialogName.value = active.title || active.url;
        if (bmDialogFolder) bmDialogFolder.value = 'Bookmarks Bar';
        if (bookmarkEditDialog) {
            bookmarkEditDialog.classList.add('show');
            expandOverlay(280);
            setTimeout(() => {
                if (bmDialogName) {
                    bmDialogName.focus();
                    bmDialogName.select();
                }
            }, 50);
        }
    }

    btnStar.addEventListener('click', openBookmarkDialog);

    if (btnBmDialogDone) {
        btnBmDialogDone.addEventListener('click', () => {
            const active = state.tabs.find(t => t.id === state.activeTabId);
            if (active && bmDialogName && bmDialogFolder) {
                sendIpc({
                    type: 'update_bookmark_details',
                    url: active.url,
                    title: bmDialogName.value.trim() || active.title,
                    folder: bmDialogFolder.value
                });
            }
            closeOverlays();
        });
    }

    if (btnBmDialogRemove) {
        btnBmDialogRemove.addEventListener('click', () => {
            const active = state.tabs.find(t => t.id === state.activeTabId);
            if (active) {
                sendIpc({
                    type: 'delete_bookmark',
                    url: active.url
                });
                window.__nexora_set_bookmarked(false);
            }
            closeOverlays();
        });
    }

    if (btnBmDialogClose) {
        btnBmDialogClose.addEventListener('click', closeOverlays);
    }

    function openClearDataDialog() {
        closeOverlays();
        if (clearDataDialog) {
            clearDataDialog.classList.add('show');
            expandOverlay(340);
        }
    }

    if (btnClearDialogCancel) btnClearDialogCancel.addEventListener('click', closeOverlays);
    if (btnClearDialogClose) btnClearDialogClose.addEventListener('click', closeOverlays);
    if (btnClearDialogConfirm) {
        btnClearDialogConfirm.addEventListener('click', () => {
            const hours = parseInt(clearRangeSelect ? clearRangeSelect.value : '0', 10);
            const clearHist = chkClearHistory ? chkClearHistory.checked : true;
            const clearDl = chkClearDownloads ? chkClearDownloads.checked : true;
            const clearCookies = chkClearCookies ? chkClearCookies.checked : true;
            sendIpc({
                type: 'clear_browsing_data',
                hours: hours,
                clear_history: clearHist,
                clear_downloads: clearDl,
                clear_cookies: clearCookies
            });
            closeOverlays();
        });
    }

    if (btnReader) {
        btnReader.addEventListener('click', () => {
            sendIpc({ type: 'toggle_reader' });
        });
    }

    btnTabMenu.addEventListener('click', (e) => {
        e.stopPropagation();
        const isOpen = tabSearchDropdown.classList.contains('show');
        closeOverlays();
        if (!isOpen) {
            renderTabSearch('');
            tabSearchDropdown.classList.add('show');
            expandOverlay(320);
            setTimeout(() => {
                tabSearchInput.focus();
            }, 50);
        }
    });

    tabSearchInput.addEventListener('input', () => {
        renderTabSearch(tabSearchInput.value);
    });

    btnMoreMenu.addEventListener('click', (e) => {
        e.stopPropagation();
        sendIpc({ type: 'open_more_menu' });
    });

    function openSecurityPopover(anchorEl) {
        const isOpen = securityPopover.classList.contains('show');
        closeOverlays();
        if (!isOpen) {
            if (anchorEl) {
                const rect = anchorEl.getBoundingClientRect();
                if (rect.left < window.innerWidth / 2) {
                    securityPopover.style.left = `${Math.max(10, rect.left)}px`;
                    securityPopover.style.right = 'auto';
                } else {
                    securityPopover.style.left = 'auto';
                    securityPopover.style.right = `${Math.max(10, window.innerWidth - rect.right)}px`;
                }
            }
            securityPopover.classList.add('show');
            expandOverlay(280);
        }
    }

    if (btnSiteInfo) {
        btnSiteInfo.addEventListener('click', (e) => {
            e.stopPropagation();
            openSecurityPopover(btnSiteInfo);
        });
    }

    if (btnExtensions && extensionsDropdown) {
        btnExtensions.addEventListener('click', (e) => {
            e.stopPropagation();
            const isOpen = extensionsDropdown.classList.contains('show');
            closeOverlays();
            if (!isOpen) {
                extensionsDropdown.classList.add('show');
                expandOverlay(280);
            }
        });
    }

    if (extShieldItem) {
        extShieldItem.addEventListener('click', (e) => {
            e.stopPropagation();
            openSecurityPopover(btnSiteInfo);
        });
    }

    if (extReaderItem) {
        extReaderItem.addEventListener('click', (e) => {
            e.stopPropagation();
            closeOverlays();
            sendIpc({ type: 'toggle_reader' });
        });
    }

    if (btnManageExtensions) {
        btnManageExtensions.addEventListener('click', (e) => {
            e.stopPropagation();
            closeOverlays();
            sendIpc({ type: 'navigate', url: 'nexora://settings' });
        });
    }

    if (btnDownloads) {
        btnDownloads.addEventListener('click', (e) => {
            e.stopPropagation();
            sendIpc({ type: 'navigate', url: 'nexora://downloads' });
        });
    }

    if (btnZoomReset) {
        btnZoomReset.addEventListener('click', (e) => {
            e.stopPropagation();
            sendIpc({ type: 'reset_zoom' });
            btnZoomReset.style.display = 'none';
        });
    }

    if (btnBack) btnBack.disabled = true;
    if (btnForward) btnForward.disabled = true;

    btnProfile.addEventListener('click', (e) => {
        e.stopPropagation();
        const isOpen = profilePopup.classList.contains('show');
        closeOverlays();
        if (!isOpen) {
            profilePopup.classList.add('show');
            expandOverlay(620);
        }
    });

    if (profBtnSignin) {
        profBtnSignin.addEventListener('click', () => {
            closeOverlays();
            sendIpc({ type: 'navigate', url: 'https://accounts.google.com/signin' });
        });
    }

    const profBtnAddAccount = document.getElementById('prof-btn-add-account');
    if (profBtnAddAccount) {
        profBtnAddAccount.addEventListener('click', () => {
            closeOverlays();
            sendIpc({ type: 'navigate', url: 'https://accounts.google.com/AddSession' });
        });
    }

    profBtnPasswords.addEventListener('click', () => {
        closeOverlays();
        sendIpc({ type: 'navigate', url: 'nexora://settings' });
    });

    profBtnGoogle.addEventListener('click', () => {
        closeOverlays();
        createNewTab('https://myaccount.google.com', 'Google Account');
    });

    let profileModalMode = 'add';
    let profileModalColor = '#4285f4';

    function updateProfileModalPreview() {
        const val = profModalName ? profModalName.value.trim() : '';
        const initial = val ? val.charAt(0).toUpperCase() : 'P';
        if (profModalAvatarInitial) {
            profModalAvatarInitial.textContent = initial;
        }
        if (profModalAvatar) {
            profModalAvatar.style.backgroundColor = profileModalColor;
        }
    }

    function selectModalColor(color) {
        profileModalColor = color || '#4285f4';
        if (profModalPalette) {
            const swatches = profModalPalette.querySelectorAll('.palette-swatch');
            swatches.forEach(swatch => {
                if (swatch.getAttribute('data-color') === profileModalColor) {
                    swatch.classList.add('active');
                } else {
                    swatch.classList.remove('active');
                }
            });
        }
        updateProfileModalPreview();
    }

    function openProfileModal(mode) {
        profileModalMode = mode;
        closeOverlays();

        const isCustomize = mode === 'customize';
        if (profModalTitle) {
            profModalTitle.textContent = isCustomize ? 'Customize profile' : 'Add profile';
        }

        const initialName = isCustomize
            ? (state.activeProfile ? state.activeProfile.name : 'Person 1')
            : `Person ${(state.profiles && state.profiles.length) ? state.profiles.length + 1 : 2}`;

        const initialColor = (isCustomize && state.activeProfile && state.activeProfile.color)
            ? state.activeProfile.color
            : '#4285f4';

        if (profModalName) {
            profModalName.value = initialName;
        }

        selectModalColor(initialColor);

        if (profileModalDialog) {
            profileModalDialog.classList.add('show');
            if (backdrop) backdrop.classList.add('show');
            expandOverlay(380);
            setTimeout(() => {
                if (profModalName) {
                    profModalName.focus();
                    profModalName.select();
                }
            }, 50);
        }
    }

    function saveProfileModal() {
        const name = profModalName ? profModalName.value.trim() : '';
        if (name) {
            if (profileModalMode === 'customize') {
                sendIpc({
                    type: 'customize_profile',
                    name: name,
                    color: profileModalColor
                });
            } else {
                sendIpc({
                    type: 'add_profile',
                    name: name,
                    email: '',
                    badge: '',
                    color: profileModalColor
                });
            }
        }
        closeOverlays();
    }

    if (profBtnCustomize) {
        profBtnCustomize.addEventListener('click', (e) => {
            e.stopPropagation();
            openProfileModal('customize');
        });
    }

    if (profBtnAdd) {
        profBtnAdd.addEventListener('click', (e) => {
            e.stopPropagation();
            openProfileModal('add');
        });
    }

    if (btnProfModalClose) btnProfModalClose.addEventListener('click', closeOverlays);
    if (btnProfModalCancel) btnProfModalCancel.addEventListener('click', closeOverlays);
    if (btnProfModalSave) btnProfModalSave.addEventListener('click', saveProfileModal);

    if (profModalName) {
        profModalName.addEventListener('input', updateProfileModalPreview);
        profModalName.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                e.preventDefault();
                saveProfileModal();
            } else if (e.key === 'Escape') {
                e.preventDefault();
                closeOverlays();
            }
        });
    }

    if (profModalPalette) {
        profModalPalette.addEventListener('click', (e) => {
            const swatch = e.target.closest('.palette-swatch');
            if (swatch) {
                const color = swatch.getAttribute('data-color');
                if (color) selectModalColor(color);
            }
        });
    }

    profBtnAccount.addEventListener('click', () => {
        closeOverlays();
        sendIpc({ type: 'navigate', url: 'nexora://settings' });
    });

    profBtnSignout.addEventListener('click', () => {
        closeOverlays();
        sendIpc({ type: 'sign_out' });
        sendIpc({ type: 'navigate', url: 'https://accounts.google.com/Logout' });
    });

    const profBtnGuest = document.getElementById('prof-btn-guest');
    if (profBtnGuest) {
        profBtnGuest.addEventListener('click', () => {
            closeOverlays();
            sendIpc({ type: 'new_incognito' });
        });
    }

    backdrop.addEventListener('click', closeOverlays);

    menuNewTab.addEventListener('click', () => {
        createNewTab();
        closeOverlays();
    });
    menuHistory.addEventListener('click', () => {
        sendIpc({ type: 'navigate', url: 'nexora://history' });
        closeOverlays();
    });
    menuBookmarks.addEventListener('click', () => {
        sendIpc({ type: 'navigate', url: 'nexora://bookmarks' });
        closeOverlays();
    });
    menuDownloads.addEventListener('click', () => {
        sendIpc({ type: 'navigate', url: 'nexora://downloads' });
        closeOverlays();
    });
    menuSettings.addEventListener('click', () => {
        sendIpc({ type: 'navigate', url: 'nexora://settings' });
        closeOverlays();
    });
    menuExit.addEventListener('click', () => {
        sendIpc({ type: 'close_window' });
    });

    zoomMinus.addEventListener('click', () => {
        window.__nexora_zoom_out();
    });
    zoomPlus.addEventListener('click', () => {
        window.__nexora_zoom_in();
    });

    btnWinMin.addEventListener('click', () => sendIpc({ type: 'minimize_window' }));
    btnWinMax.addEventListener('click', () => sendIpc({ type: 'toggle_maximize_window' }));
    btnWinClose.addEventListener('click', () => sendIpc({ type: 'close_window' }));

    dragSpacer.addEventListener('mousedown', (e) => {
        if (e.button === 0) sendIpc({ type: 'drag_window' });
    });

    dragSpacer.addEventListener('dblclick', () => {
        sendIpc({ type: 'toggle_maximize_window' });
    });

    tabStripRow.addEventListener('mousedown', (e) => {
        if (e.target === tabStripRow || e.target === tabStrip) {
            if (e.button === 0) sendIpc({ type: 'drag_window' });
        }
    });

    if (tabContextMenu) {
        tabContextMenu.querySelectorAll('.menu-item').forEach(item => {
            item.addEventListener('click', () => {
                const action = item.dataset.action;
                const targetId = state.contextTabId || state.activeTabId;
                closeOverlays();
                if (window.__nexora_tab_context_action) {
                    window.__nexora_tab_context_action(action, targetId);
                }
            });
        });
    }

    window.addEventListener('keydown', (e) => {
        if (e.ctrlKey && e.key === 't') {
            e.preventDefault();
            createNewTab();
        } else if (e.ctrlKey && e.key === 'w') {
            e.preventDefault();
            closeTab(state.activeTabId);
        } else if (e.ctrlKey && e.shiftKey && (e.key === 'T' || e.key === 't')) {
            e.preventDefault();
            sendIpc({ type: 'reopen_closed_tab' });
        } else if (e.ctrlKey && e.shiftKey && (e.key === 'b' || e.key === 'B')) {
            e.preventDefault();
            window.__nexora_toggle_bookmarks_bar_visibility();
        } else if (e.ctrlKey && e.shiftKey && (e.key === 'Delete' || e.key === 'Del')) {
            e.preventDefault();
            openClearDataDialog();
        } else if (e.ctrlKey && (e.key === 'd' || e.key === 'D')) {
            e.preventDefault();
            openBookmarkDialog();
        } else if (e.ctrlKey && (e.key === '=' || e.key === '+')) {
            e.preventDefault();
            window.__nexora_zoom_in();
        } else if (e.ctrlKey && (e.key === '-' || e.key === '_')) {
            e.preventDefault();
            window.__nexora_zoom_out();
        } else if (e.ctrlKey && e.key === '0') {
            e.preventDefault();
            window.__nexora_zoom_reset();
        } else if (e.ctrlKey && (e.key === 'r' || e.key === 'R')) {
            e.preventDefault();
            sendIpc({ type: 'reload' });
        } else if (e.key === 'F5') {
            e.preventDefault();
            sendIpc({ type: 'reload' });
        } else if (e.ctrlKey && (e.key === 'l' || e.key === 'L')) {
            e.preventDefault();
            omniboxInput.focus();
            omniboxInput.select();
        } else if (e.ctrlKey && (e.key === 'h' || e.key === 'H')) {
            e.preventDefault();
            sendIpc({ type: 'navigate', url: 'nexora://history' });
        } else if (e.ctrlKey && (e.key === 'j' || e.key === 'J')) {
            e.preventDefault();
            sendIpc({ type: 'navigate', url: 'nexora://downloads' });
        } else if (e.ctrlKey && e.shiftKey && (e.key === 'O' || e.key === 'o')) {
            e.preventDefault();
            sendIpc({ type: 'navigate', url: 'nexora://bookmarks' });
        } else if (e.ctrlKey && (e.key === 'f' || e.key === 'F')) {
            e.preventDefault();
            toggleFindBar();
        } else if (e.altKey && (e.key === 'r' || e.key === 'R')) {
            e.preventDefault();
            sendIpc({ type: 'toggle_reader' });
        } else if (e.key === 'F11') {
            e.preventDefault();
            sendIpc({ type: 'toggle_fullscreen' });
        } else if (e.ctrlKey && e.key >= '1' && e.key <= '8') {
            e.preventDefault();
            const idx = parseInt(e.key, 10) - 1;
            if (idx < state.tabs.length) {
                switchTab(state.tabs[idx].id);
            }
        } else if (e.ctrlKey && e.key === '9') {
            e.preventDefault();
            if (state.tabs.length > 0) {
                switchTab(state.tabs[state.tabs.length - 1].id);
            }
        } else if (e.ctrlKey && !e.shiftKey && e.key === 'Tab') {
            e.preventDefault();
            const currentIdx = state.tabs.findIndex(t => t.id === state.activeTabId);
            const nextIdx = (currentIdx + 1) % state.tabs.length;
            switchTab(state.tabs[nextIdx].id);
        } else if (e.ctrlKey && e.shiftKey && e.key === 'Tab') {
            e.preventDefault();
            const currentIdx = state.tabs.findIndex(t => t.id === state.activeTabId);
            const prevIdx = (currentIdx - 1 + state.tabs.length) % state.tabs.length;
            switchTab(state.tabs[prevIdx].id);
        } else if (e.key === 'Escape') {
            closeOverlays();
        }
    });

    window.__nexora_create_new_tab = createNewTab;
    window.__nexora_close_active_tab = function() {
        closeTab(state.activeTabId);
    };
    window.__nexora_focus_omnibox = function() {
        omniboxInput.focus();
        omniboxInput.select();
    };
    window.__nexora_toggle_find_bar = toggleFindBar;
    window.__nexora_update_find_count = function(text) {
        if (findCount) {
            findCount.textContent = text || '';
        }
    };
    window.__nexora_open_clear_data_dialog = openClearDataDialog;
    window.__nexora_open_bookmark_dialog = openBookmarkDialog;

    window.__nexora_render_bookmarks_bar = function(items) {
        if (!bookmarksBar) return;
        bookmarksBar.innerHTML = '';
        if (!Array.isArray(items) || items.length === 0) {
            const empty = document.createElement('span');
            empty.style.color = 'var(--text-muted)';
            empty.style.fontSize = '11px';
            empty.style.padding = '0 6px';
            empty.textContent = 'For quick access, place your bookmarks here on the bookmarks bar.';
            bookmarksBar.appendChild(empty);
            return;
        }
        items.forEach(item => {
            const bItem = document.createElement('div');
            bItem.className = 'bookmark-bar-item';
            bItem.title = `${item.title}\n${item.url}`;
            const domain = getDomain(item.url);
            const s2Url = domain ? `https://www.google.com/s2/favicons?domain=${domain}&sz=32` : '';
            const iconSrc = s2Url || NEXORA_LOGO_DATA_URI;
            bItem.innerHTML = `
                <img src="${escapeHtml(iconSrc)}" class="bookmark-bar-icon" alt="" onerror="this.src='${NEXORA_LOGO_DATA_URI}';" />
                <span class="bookmark-bar-title">${escapeHtml(item.title || item.url)}</span>
            `;
            bItem.addEventListener('click', (e) => {
                e.stopPropagation();
                navigateOmnibox(item.url);
            });
            bookmarksBar.appendChild(bItem);
        });
    };

    window.__nexora_toggle_bookmarks_bar_visibility = function(show) {
        if (!bookmarksBar) return;
        if (typeof show === 'boolean') {
            bookmarksBar.classList.toggle('hidden', !show);
        } else {
            bookmarksBar.classList.toggle('hidden');
        }
        const isVisible = !bookmarksBar.classList.contains('hidden');
        const chromeEl = document.querySelector('.nexora-chrome');
        if (chromeEl) {
            chromeEl.classList.toggle('has-bookmarks-bar', isVisible);
        }
        sendIpc({
            type: 'set_setting',
            key: 'show_bookmarks_bar',
            value: isVisible ? 'true' : 'false'
        });
        sendIpc({
            type: 'bookmarks_bar_toggled',
            visible: isVisible
        });
    };

    window.__nexora_update_tab_audio = function(tabId, isAudible, isMuted) {
        const tab = state.tabs.find(t => t.id === tabId);
        if (tab) {
            tab.isAudible = isAudible;
            tab.isMuted = isMuted;
            renderTabs();
        }
    };

    window.__nexora_zoom_in = function() {
        if (state.zoomLevel < 200) {
            state.zoomLevel += 10;
            zoomLevelEl.textContent = state.zoomLevel + '%';
            window.__nexora_set_zoom_level(state.zoomLevel);
            sendIpc({ type: 'set_zoom', level: state.zoomLevel });
        }
    };

    window.__nexora_zoom_out = function() {
        if (state.zoomLevel > 50) {
            state.zoomLevel -= 10;
            zoomLevelEl.textContent = state.zoomLevel + '%';
            window.__nexora_set_zoom_level(state.zoomLevel);
            sendIpc({ type: 'set_zoom', level: state.zoomLevel });
        }
    };

    window.__nexora_zoom_reset = function() {
        state.zoomLevel = 100;
        zoomLevelEl.textContent = '100%';
        window.__nexora_set_zoom_level(100);
        sendIpc({ type: 'set_zoom', level: 100 });
    };

    window.__nexora_toggle_more_menu = function() {
        const isOpen = moreMenuDropdown.classList.contains('show');
        closeOverlays();
        if (!isOpen) {
            moreMenuDropdown.classList.add('show');
            expandOverlay(360);
        }
    };

    window.__nexora_update_url = function(url) {
        if (!url || url.startsWith('data:') || url === 'about:blank') {
            return;
        }
        const active = state.tabs.find(t => t.id === state.activeTabId);
        if (active) {
            active.url = url;
            omniboxInput.value = (url === 'nexora://home') ? '' : url;
            if (url === 'nexora://history') active.title = 'History - Nexora';
            else if (url === 'nexora://bookmarks') active.title = 'Bookmarks - Nexora';
            else if (url === 'nexora://settings') active.title = 'Settings - Nexora';
            else if (url === 'nexora://downloads') active.title = 'Downloads - Nexora';
            renderTabs();
            syncTabsToDatabase();
        }
    };

    window.__nexora_update_tab_title = function(tabId, title) {
        const tab = state.tabs.find(t => t.id === tabId);
        if (tab && title && title !== 'Untitled' && title !== 'about:blank') {
            tab.title = title;
            renderTabs();
            syncTabsToDatabase();
        }
    };

    window.__nexora_update_title = function(title) {
        window.__nexora_update_tab_title(state.activeTabId, title);
    };

    window.__nexora_update_tab_favicon = function(tabId, faviconUrl) {
        const tab = state.tabs.find(t => t.id === tabId);
        if (tab && faviconUrl && !faviconUrl.includes('null/favicon.ico')) {
            tab.favicon = faviconUrl;
            renderTabs();
            syncTabsToDatabase();
        }
    };

    window.__nexora_page_loading = function(isLoading) {
        if (!pageLoadingBar) return;
        if (isLoading) {
            pageLoadingBar.classList.remove('finished');
            pageLoadingBar.classList.add('loading');
        } else {
            pageLoadingBar.classList.remove('loading');
            pageLoadingBar.classList.add('finished');
            setTimeout(() => {
                pageLoadingBar.classList.remove('finished');
            }, 300);
        }
    };

    window.__nexora_restore_tab = function(tabId, url, title) {
        const id = tabId || ('tab-' + Date.now());
        const newTab = { id: id, title: title || 'Nexora Home', url: url || 'nexora://home', favicon: '', pinned: false };
        state.tabs.push(newTab);
        state.activeTabId = id;
        omniboxInput.value = (url === 'nexora://home') ? '' : url;
        renderTabs();
    };

    window.__nexora_update_shield = function(count) {
        state.shieldCount = count;
        if (shieldCount) shieldCount.textContent = count;
        if (popoverShieldVal) popoverShieldVal.textContent = `Active (${count} blocked)`;
        if (shieldExtStatus) shieldExtStatus.innerHTML = `Protection Active (<span id="shield-count">${count}</span> blocked)`;
    };

    window.__nexora_set_bookmarked = function(isBookmarked) {
        state.isBookmarked = isBookmarked;
        btnStar.classList.toggle('bookmarked', isBookmarked);
    };

    window.__nexora_set_reader_mode = function(isActive) {
        state.isReaderMode = isActive;
        if (btnReader) btnReader.classList.toggle('active', isActive);
    };

    window.__nexora_set_incognito = function(isIncognito) {
        state.isIncognito = isIncognito;
        document.body.classList.toggle('incognito', isIncognito);
    };

    window.__nexora_update_settings = function(settings) {
        if (!settings) return;
        if (settings.search_engine) {
            state.searchEngine = settings.search_engine;
        }
    };

    window.__nexora_set_zoom_level = function(percent) {
        if (!btnZoomReset || !zoomBadgeText) return;
        if (percent === 100) {
            btnZoomReset.style.display = 'none';
        } else {
            zoomBadgeText.textContent = `${percent}%`;
            btnZoomReset.style.display = 'inline-flex';
        }
    };

    window.__nexora_update_nav_state = function(canGoBack, canGoForward) {
        if (btnBack) btnBack.disabled = !canGoBack;
        if (btnForward) btnForward.disabled = !canGoForward;
    };

    window.__nexora_render_omnibox_suggestions = function(suggestions) {
        renderOmniboxSuggestions(suggestions);
    };

    window.__nexora_update_profiles = function(profiles) {
        if (!Array.isArray(profiles) || profiles.length === 0) return;
        state.profiles = profiles;
        state.activeProfile = profiles.find(p => p.is_active) || profiles[0];

        const hasEmail = !!(state.activeProfile.email && state.activeProfile.email.trim());
        const initial = (state.activeProfile.name || 'P').charAt(0).toUpperCase();
        const profileColor = state.activeProfile.color || '#4285f4';

        if (toolbarAvatarInitial && toolbarAvatarSvg) {
            toolbarAvatarInitial.textContent = initial;
            toolbarAvatarInitial.style.display = 'block';
            toolbarAvatarSvg.style.display = 'none';
            toolbarAvatarInitial.parentElement.style.background = profileColor;
        }

        if (popupAvatarInitial && popupAvatarSvg) {
            popupAvatarInitial.textContent = initial;
            popupAvatarInitial.style.display = 'block';
            popupAvatarSvg.style.display = 'none';
            popupAvatarCircle.style.background = profileColor;
        }

        if (popupProfileName) {
            popupProfileName.textContent = state.activeProfile.name || 'Person 1';
        }

        if (popupProfileEmail) {
            popupProfileEmail.textContent = hasEmail ? state.activeProfile.email : 'Not signed in';
        }

        if (popupProfileBadge) {
            if (hasEmail && state.activeProfile.badge) {
                popupProfileBadge.textContent = state.activeProfile.badge;
                popupProfileBadge.style.display = 'block';
            } else {
                popupProfileBadge.style.display = 'none';
            }
        }

        if (profBtnSignin) {
            profBtnSignin.style.display = hasEmail ? 'none' : 'flex';
        }

        if (profBtnGoogle) {
            profBtnGoogle.style.display = hasEmail ? 'flex' : 'none';
        }

        const profBtnAddAccount = document.getElementById('prof-btn-add-account');
        if (profBtnAddAccount) {
            profBtnAddAccount.style.display = hasEmail ? 'flex' : 'none';
        }

        if (profBtnSignout) {
            profBtnSignout.style.display = hasEmail ? 'flex' : 'none';
        }

        const others = profiles.filter(p => p.id !== state.activeProfile.id);
        if (profileOtherSection) {
            profileOtherSection.style.display = others.length > 0 ? 'block' : 'none';
        }

        if (profileOtherList) {
            profileOtherList.innerHTML = '';
            others.forEach(p => {
                const row = document.createElement('div');
                row.className = 'profile-item-row';
                const initial = (p.name || 'P').charAt(0).toUpperCase();
                const color = p.color || '#4285f4';
                row.innerHTML = `
                    <div class="profile-circle-mini" style="background: ${color};">
                        <span>${initial}</span>
                    </div>
                    <div style="display: flex; flex-direction: column; min-width: 0; flex: 1;">
                        <span class="profile-item-name">${escapeHtml(p.name)}</span>
                        ${p.email ? `<span style="font-size: 11px; color: #9aa0a6; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">${escapeHtml(p.email)}</span>` : ''}
                    </div>
                `;
                row.addEventListener('click', () => {
                    closeOverlays();
                    sendIpc({ type: 'switch_profile', id: p.id });
                });
                profileOtherList.appendChild(row);
            });
        }
    };

    window.__nexora_tab_context_action = function(action, tabId) {
        const targetId = tabId || state.activeTabId;
        const index = state.tabs.findIndex(t => t.id === targetId);
        if (index === -1) return;

        switch (action) {
            case 'new_tab': {
                const targetTab = state.tabs[index];
                const newId = 'tab-' + Date.now();
                const newTab = {
                    id: newId,
                    title: 'Nexora Home',
                    url: 'nexora://home',
                    favicon: '',
                    pinned: false,
                    groupId: targetTab ? targetTab.groupId : null,
                    createdAt: Date.now()
                };
                state.tabs.splice(index + 1, 0, newTab);
                state.activeTabId = newId;
                omniboxInput.value = '';
                renderTabs();
                syncTabsToDatabase();
                sendIpc({ type: 'new_tab', id: newId });
                break;
            }
            case 'duplicate': {
                const targetTab = state.tabs[index];
                const newId = 'tab-' + Date.now();
                const newTab = {
                    id: newId,
                    title: targetTab.title,
                    url: targetTab.url,
                    favicon: targetTab.favicon,
                    pinned: false,
                    groupId: targetTab.groupId || null,
                    createdAt: Date.now()
                };
                state.tabs.splice(index + 1, 0, newTab);
                state.activeTabId = newId;
                omniboxInput.value = (newTab.url === 'nexora://home') ? '' : newTab.url;
                renderTabs();
                syncTabsToDatabase();
                sendIpc({ type: 'new_tab', id: newId });
                if (newTab.url && newTab.url !== 'nexora://home') {
                    sendIpc({ type: 'navigate', url: newTab.url });
                }
                break;
            }
            case 'reload': {
                if (targetId === state.activeTabId) {
                    sendIpc({ type: 'reload' });
                } else {
                    switchTab(targetId);
                    sendIpc({ type: 'reload' });
                }
                break;
            }
            case 'pin': {
                state.tabs[index].pinned = !state.tabs[index].pinned;
                renderTabs();
                syncTabsToDatabase();
                break;
            }
            case 'add_to_new_group': {
                openTabGroupDialog(null, targetId);
                break;
            }
            case 'remove_from_group': {
                const targetTab = state.tabs.find(t => t.id === targetId);
                if (targetTab) {
                    targetTab.groupId = null;
                    renderTabs();
                    syncTabsToDatabase();
                }
                break;
            }
            case 'close': {
                closeTab(targetId);
                break;
            }
            case 'close_others': {
                const kept = state.tabs[index];
                const toClose = state.tabs.filter(t => t.id !== targetId);
                toClose.forEach(t => sendIpc({ type: 'close_tab', id: t.id }));
                state.tabs = [kept];
                state.activeTabId = kept.id;
                switchTab(kept.id);
                syncTabsToDatabase();
                break;
            }
            case 'close_right': {
                const toClose = state.tabs.slice(index + 1);
                toClose.forEach(t => sendIpc({ type: 'close_tab', id: t.id }));
                state.tabs = state.tabs.slice(0, index + 1);
                if (!state.tabs.find(t => t.id === state.activeTabId)) {
                    state.activeTabId = state.tabs[state.tabs.length - 1].id;
                    switchTab(state.activeTabId);
                } else {
                    renderTabs();
                }
                syncTabsToDatabase();
                break;
            }
        }
    };

    window.__nexora_restore_tab_groups = function(groups) {
        if (Array.isArray(groups)) {
            state.groups = groups;
            renderTabs();
        }
    };

    window.__nexora_restore_tabs = function(tabs) {
        if (Array.isArray(tabs) && tabs.length > 0) {
            state.tabs = tabs.map(t => ({
                id: t.id,
                title: t.title || 'Nexora Home',
                url: t.url || 'nexora://home',
                favicon: t.favicon || '',
                pinned: !!t.pinned,
                groupId: t.group_id || null,
                createdAt: t.created_at || Date.now()
            }));
            const active = tabs.find(t => t.is_active) || tabs[0];
            state.activeTabId = active ? active.id : tabs[0].id;
            const curTab = state.tabs.find(t => t.id === state.activeTabId);
            if (curTab) {
                omniboxInput.value = (curTab.url === 'nexora://home' || curTab.url === '') ? '' : curTab.url;
            }
            renderTabs();
        }
    };

    sendIpc({ type: 'get_profiles' });
    renderTabs();
})();
