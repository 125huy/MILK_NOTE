<script>
  import { onMount } from 'svelte';
  import { appState, appError, hydrateAppState, setTheme, setTypography } from './lib/app-state.js';
  import { api, runtime } from './lib/api.js';

  const pages = ['Trang chủ', 'Ghi chú', 'Thẻ', 'Ôn tập', 'Task', 'Cài đặt'];
  let page = 'Trang chủ';
  let notes = [
    { path: 'Daily/2026-10-24.md', title: 'Nhật ký hôm nay', content: '# Nhật ký hôm nay\n\n- [ ] Hoàn thiện MilkNote\n- [ ] Ôn lại Rust\n\n## Ghi chú\nMilkNote là nơi giữ những điều đáng nhớ.' },
    { path: 'Hoc-tap/Rust.md', title: 'Rust', content: '# Rust\n\nRust giúp viết phần mềm an toàn và nhanh.\n\n- [ ] Đọc ownership\n- [x] Viết test đầu tiên' }
  ];
  let cards = [{ title: 'Ownership là gì?', content: 'Cơ chế quản lý quyền sở hữu bộ nhớ của Rust.', color: 'milk', review: true }, { title: 'Công thức diện tích', content: '$S = a \\times b$', color: 'butter', review: false }];
  let selected = null;
  let draft = '';
  let editing = false;
  let preview = false;
  let showSettings = false;
  let selectedFont = 'be-vietnam-pro';
  let selectedScale = 100;
  let reviewIndex = 0;
  let revealed = false;

  onMount(async () => {
    try { const state = await hydrateAppState(); selectedFont = state.fontFamily; selectedScale = state.fontScale; } catch (_) {}
  });

  function openNote(note) { selected = note; draft = note.content; editing = true; preview = false; page = 'Ghi chú'; }
  function newNote() { const note = { path: `Ghi-chu-${notes.length + 1}.md`, title: 'Ghi chú mới', content: '# Ghi chú mới\n\n' }; notes = [...notes, note]; openNote(note); }
  async function saveNote() {
    if (!selected) return;
    try {
      if (runtime === 'tauri') await api.noteUpdate(selected.path, draft);
      selected.content = draft;
      notes = [...notes];
    } catch (error) { alert(String(error)); }
  }
  function closeEditor() { editing = false; selected = null; }
  function escapeHtml(value) { return value.replace(/[&<>"']/g, (c) => ({ '&':'&amp;', '<':'&lt;', '>':'&gt;', '"':'&quot;', "'":'&#39;' }[c])); }
  function renderMarkdown(value) {
    return escapeHtml(value).replace(/^### (.*)$/gm, '<h3>$1</h3>').replace(/^## (.*)$/gm, '<h2>$1</h2>').replace(/^# (.*)$/gm, '<h1>$1</h1>').replace(/^- \[ \] (.*)$/gm, '<p>□ $1</p>').replace(/^- \[x\] (.*)$/gm, '<p>☑ $1</p>').replace(/^[-*] (.*)$/gm, '<p>• $1</p>').replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>').replace(/`([^`]+)`/g, '<code>$1</code>').replace(/\$([^$]+)\$/g, '<span class="math">$1</span>').replace(/\n{2,}/g, '<br><br>').replace(/\n/g, '<br>');
  }
  $: taskItems = notes.flatMap((note) => [...note.content.matchAll(/^- \[([ x])\] (.+)$/gm)].map((match) => ({ done: match[1] === 'x', text: match[2], source: note.title })));
  $: reviewCards = cards.filter((card) => card.review);
  async function changeTheme(theme) { await setTheme(theme); }
  async function changeTypography() { await setTypography(selectedFont, Number(selectedScale)); }
</script>

<svelte:head><title>MilkNote</title></svelte:head>

<div class="app-shell theme-{$appState.theme}" style={`--font-scale: ${$appState.fontScale / 100};`}>
  <aside class="sidebar">
    <div class="brand"><span class="brand-mark">M</span><span>MilkNote</span></div>
    <button class="new-note" on:click={newNote}>＋ Ghi chú mới <span>Ctrl+N</span></button>
    <nav aria-label="Điều hướng">{#each pages as item}<button class:active={page === item} on:click={() => { page = item; editing = false; }}>{item}{item === 'Ôn tập' && reviewCards.length ? ` · ${reviewCards.length}` : ''}</button>{/each}</nav>
    <div class="sidebar-note">Offline-first<br /><small>IPC: {runtime === 'tauri' ? 'Tauri v2' : 'Mock dev/test'}</small></div>
  </aside>

  <main class="content">
    <header class="topbar"><div><p class="eyebrow">VÙNG HỌC TẬP CỦA BẠN</p><h1>{editing ? selected.title : page}</h1></div><div class="top-actions"><span class="save-state">{editing ? '● Bản nháp cục bộ' : '● Sẵn sàng'}</span><button class="icon-button" aria-label="Cài đặt" on:click={() => { page = 'Cài đặt'; editing = false; }}>⚙</button></div></header>
    {#if $appState.loading}<div class="notice">Đang nạp trạng thái ứng dụng…</div>{/if}
    {#if $appError}<div class="notice error">{$appError}</div>{/if}

    {#if editing}
      <section class="editor-shell">
        <div class="editor-toolbar"><button class:chosen={!preview} on:click={() => preview = false}>Viết</button><button class:chosen={preview} on:click={() => preview = true}>Xem trước</button><span class="toolbar-spacer"></span><button on:click={saveNote}>Lưu <kbd>Ctrl+S</kbd></button><button on:click={closeEditor}>Đóng</button></div>
        {#if preview}<article class="markdown-preview">{@html renderMarkdown(draft)}</article>{:else}<textarea class="editor" bind:value={draft} spellcheck="true" aria-label="Nội dung Markdown"></textarea>{/if}
        <footer class="editor-footer"><span>{draft.length} ký tự · {draft.trim() ? draft.trim().split(/\s+/).length : 0} từ</span><span>{selected.path}</span></footer>
      </section>
    {:else if page === 'Trang chủ'}
      <section class="welcome-card"><div><p class="eyebrow">HÔM NAY</p><h2>Viết xuống để nhớ lâu hơn.</h2><p class="muted">Ghi chú Markdown cục bộ, an toàn và dễ mở bằng trình soạn thảo khác.</p><button class="primary" on:click={newNote}>Tạo ghi chú đầu tiên</button></div><div class="paper-lines" aria-hidden="true"></div></section>
      <section class="dashboard-grid"><div class="panel"><div class="panel-title"><h3>Gần đây</h3><button on:click={() => page = 'Ghi chú'}>Xem tất cả</button></div>{#each notes.slice(0, 4) as note}<button class="list-row" on:click={() => openNote(note)}><span class="row-icon">✎</span><span><strong>{note.title}</strong><small>{note.path}</small></span></button>{/each}</div><div class="panel"><div class="panel-title"><h3>Task hôm nay</h3><span>{taskItems.filter((item) => !item.done).length} việc</span></div>{#each taskItems.slice(0, 4) as task}<div class="task-row"><span class:done={task.done}>{task.done ? '☑' : '□'}</span><span>{task.text}</span></div>{/each}</div></section>
    {:else if page === 'Ghi chú'}
      <section class="panel notes-panel"><div class="panel-title"><h3>Tất cả ghi chú</h3><button class="primary small" on:click={newNote}>＋ Mới</button></div>{#each notes as note}<button class="note-card-row" on:click={() => openNote(note)}><span class="note-symbol">#</span><span><strong>{note.title}</strong><small>{note.path} · {note.content.length} ký tự</small></span><span>›</span></button>{/each}</section>
    {:else if page === 'Task'}
      <section class="panel"><div class="panel-title"><h3>Task tổng hợp</h3><span>{taskItems.filter((item) => !item.done).length} chưa xong</span></div>{#each taskItems as task}<div class="task-row large"><span class:done={task.done}>{task.done ? '☑' : '□'}</span><span><strong>{task.text}</strong><small>{task.source}</small></span></div>{/each}</section>
    {:else if page === 'Thẻ'}
      <section class="cards-grid">{#each cards as card}<article class={`flash-card ${card.color}`}><div class="card-head"><span>NOTE CARD</span><button>⋯</button></div><h3>{card.title}</h3><p>{card.content}</p><footer><span>{card.review ? 'Ôn tập bật' : 'Ghi chú'}</span><button on:click={() => card.review = !card.review}>{card.review ? 'Tắt ôn tập' : 'Bật ôn tập'}</button></footer></article>{/each}<button class="add-card" on:click={() => cards = [...cards, { title: 'Thẻ mới', content: 'Nội dung cần nhớ…', color: 'cream', review: true }]}>＋ Tạo thẻ</button></section>
    {:else if page === 'Ôn tập'}
      <section class="review-panel">{#if reviewCards.length}<p class="eyebrow">THẺ {reviewIndex + 1} / {reviewCards.length}</p><button class="review-card" on:click={() => revealed = !revealed} aria-label="Lật thẻ ôn tập"><span class="review-label">{revealed ? 'ĐÁP ÁN' : 'CÂU HỎI'}</span><h2>{revealed ? reviewCards[reviewIndex % reviewCards.length].content : reviewCards[reviewIndex % reviewCards.length].title}</h2><p>Nhấn để lật thẻ · Space</p></button><div class="rating-row">{#each ['Lại', 'Khó', 'Tốt', 'Dễ'] as rating}<button on:click={() => { reviewIndex = (reviewIndex + 1) % reviewCards.length; revealed = false; }}>{rating}</button>{/each}</div>{:else}<div class="empty-state"><div class="empty-icon">✓</div><h3>Không còn thẻ đến hạn</h3><p>Hãy thêm thẻ hoặc bật ôn tập cho một Note Card.</p></div>{/if}</section>
    {:else if page === 'Cài đặt'}
      <section class="settings-card full"><h3>Giao diện</h3><div class="setting-block"><strong>Theme</strong><div class="setting-actions"><button class:chosen={$appState.theme === 'paper'} on:click={() => changeTheme('paper')}>Milk Paper</button><button class:chosen={$appState.theme === 'cocoa'} on:click={() => changeTheme('cocoa')}>Cocoa Night</button></div></div><label>Font chữ<select bind:value={selectedFont} on:change={changeTypography}><option value="be-vietnam-pro">Be Vietnam Pro</option><option value="noto-serif">Noto Serif</option><option value="inter">Inter</option></select></label><label>Cỡ chữ<select bind:value={selectedScale} on:change={changeTypography}><option value="90">90%</option><option value="100">100%</option><option value="110">110%</option><option value="125">125%</option><option value="140">140%</option></select></label><div class="setting-block"><strong>Dữ liệu</strong><p class="muted">Vault hiện tại: {$appState.vaultPath ?? 'Chưa mở vault trong desktop shell'}</p><p class="muted">MilkNote luôn ưu tiên dữ liệu Markdown và hoạt động offline.</p></div></section>
    {/if}
  </main>
</div>
