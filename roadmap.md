# Pixi — 2‑Week Roadmap 🖌️

*Pixel art editor with a focus on powerful palette organization.*  
Target platforms: **Linux (primary)** → **Windows** → **macOS**  
Tech stack: **Rust · eframe/egui · wgpu**

---

## 🎯 Project Goal

Within **2 weeks**, Pixi should be a **usable and extensible pixel art editor**:
- Draw and erase pixel art
- Work with multiple layers
- Organize colors in **palette folders** (core differentiator)
- Save/load a custom `.pixi` project file
- Export artwork to PNG

The result should be a **solid v0.1** that can be extended later.

---

## 🧭 Roadmap Overview

- **Week 1:** Core editor functionality (“It works”)
- **Week 2:** UX polish + palette folders + stability (“Feels like a tool”)

Each day has a *clear deliverable* to keep scope under control.

---

## 📅 Week 1 — Core Editor (MVP)

### Day 1 — Project Setup & Skeleton
- [x] Create Rust project (`cargo new pixi`)
- [x] Integrate `eframe` / `egui`
- [ ] Window with basic layout:
  - Menu bar
  - Toolbar (left)
  - Side panels (right)
  - Canvas (center)
- [ ] Separate state:
  - `Project` (data/model)
  - `UiState` (view/input)

**Deliverable:** App launches with editor layout.

---

### Day 2 — Canvas & Navigation
- [ ] Canvas widget with fixed pixel resolution
- [ ] Pan (mouse drag / space + drag)
- [ ] Zoom (mouse wheel, centered on cursor)
- [ ] World ↔ screen coordinate helpers
- [ ] Checkerboard background
- [ ] Optional pixel grid at high zoom

**Deliverable:** Navigable canvas with correct coordinates.

---

### Day 3 — Pixel Drawing (Single Layer)
- [ ] RGBA pixel buffer for one layer
- [ ] Pencil tool (click + drag)
- [ ] Eraser tool
- [ ] Active color selection
- [ ] Clamp drawing to bounds

**Deliverable:** You can draw pixel art.

---

### Day 4 — Rendering & Performance
- [ ] Composite image buffer
- [ ] Upload to `egui::TextureHandle`
- [ ] Re-upload texture only when “dirty”
- [ ] Crisp nearest‑neighbor scaling

**Deliverable:** Smooth, sharp pixel rendering.

---

### Day 5 — Layers v1
- [ ] Multiple layers
- [ ] Show / hide layers
- [ ] Active layer selection
- [ ] Add / delete layer
- [ ] Reorder layers (buttons)

**Deliverable:** Proper multi‑layer editing.

---

### Day 6 — Export to PNG
- [ ] Flatten visible layers
- [ ] Export via `image` crate
- [ ] Native file dialog (`rfd`)
- [ ] Transparency preserved

**Deliverable:** Artwork exports correctly.

---

### Day 7 — Save / Load `.pixi` (v1)
- [ ] Define `.pixi` file format (versioned)
- [ ] Save/load:
  - Canvas size
  - Layers (name, visibility, pixels)
  - Palette (flat, no folders yet)
- [ ] Open saved project and continue editing

**Deliverable:** Projects persist correctly.

---

## 📅 Week 2 — UX, Palette Folders & Polish

### Day 8 — Undo / Redo
- [ ] Command stack
- [ ] Stroke-based undo (group pixel changes)
- [ ] Undo / redo shortcuts
- [ ] Support pencil + eraser

**Deliverable:** Mistakes are reversible.

---

### Day 9 — Tool Improvements
- [ ] Eyedropper tool
- [ ] Brush sizes (1 / 2 / 3 / 5)
- [ ] Straight line drawing (Shift)
- [ ] Temporary eyedropper (right-click)

**Deliverable:** Tools feel usable.

---

### Day 10 — Palette Folders (Data Model)
- [ ] Palette tree structure:
  - Folders
  - Colors
- [ ] Create / rename / delete folders
- [ ] Assign colors to folders

**Deliverable:** Palette hierarchy exists in data model.

---

### Day 11 — Palette Folders (UI)
- [ ] Tree UI (collapsing folders)
- [ ] Drag & drop colors between folders
- [ ] Reorder colors
- [ ] Context menus (add / delete / rename)

**Deliverable:** Palette organization works and feels good.

---

### Day 12 — File Format v2
- [ ] Versioned `.pixi` loader
- [ ] Compress layer pixel data (zstd)
- [ ] Backward compatibility with v1
- [ ] Optional autosave / recovery file

**Deliverable:** Robust and future-proof save format.

---

### Day 13 — Editor Polish
- [ ] Rename layers inline
- [ ] Highlight active layer
- [ ] Grid toggle
- [ ] Background toggle
- [ ] Status bar:
  - Tool
  - Zoom %
  - Cursor coordinates

**Deliverable:** Editor feels professional.

---

### Day 14 — Stabilization & Release
- [ ] Bug fixing
- [ ] Edge cases (bounds, zoom extremes)
- [ ] Manual regression testing
- [ ] Basic README (controls, features)
- [ ] Linux release build

**Deliverable:** **Pixi v0.1** 🎉

---

## 🚫 Explicitly Out of Scope (for v0.1)

- Animation / timeline
- Advanced blend modes
- Selections / marquee tools
- Tilesets
- Plugins / scripting

*(These are great future milestones, but not now.)*

---

## 🌱 Possible Post‑v0.1 Ideas
- Animation support
- GIF export
- Flood fill
- PNG import as layer
- Tilemap mode
- Aseprite palette import

---

Happy hacking — and remember: **finish first, optimize later.**
