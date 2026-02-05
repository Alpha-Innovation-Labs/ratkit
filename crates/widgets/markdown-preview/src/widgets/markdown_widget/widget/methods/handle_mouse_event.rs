//! Handle mouse events for the markdown widget.

use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

use crate::widgets::markdown_widget::extensions::scrollbar::{
    click_to_offset, is_in_scrollbar_area,
};
use crate::widgets::markdown_widget::extensions::selection::should_render_line;
use crate::widgets::markdown_widget::extensions::toc::Toc;
use crate::widgets::markdown_widget::foundation::elements::render;
use crate::widgets::markdown_widget::foundation::elements::ElementKind;
use crate::widgets::markdown_widget::foundation::events::MarkdownEvent;
use crate::widgets::markdown_widget::foundation::helpers::is_in_area;
use crate::widgets::markdown_widget::foundation::parser::render_markdown_to_elements;
use crate::widgets::markdown_widget::foundation::types::SelectionPos;
use crate::widgets::markdown_widget::state::toc_state::TocState;
use crate::widgets::markdown_widget::widget::enums::MarkdownWidgetMode;
use crate::widgets::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Internal mouse event handler with all logic.
    pub(crate) fn handle_mouse_internal(
        &mut self,
        event: &MouseEvent,
        area: Rect,
    ) -> MarkdownEvent {
        if !is_in_area(event.column, event.row, area) {
            // Click outside area exits selection mode
            if self.selection.is_active() {
                self.selection.exit();
                return MarkdownEvent::SelectionEnded;
            }
            return MarkdownEvent::None;
        }

        let border_offset = if self.bordered { 1 } else { 0 };
        let relative_y = event.row.saturating_sub(area.y + border_offset) as usize;
        let relative_x = event.column.saturating_sub(area.x) as usize;
        let width = area.width as usize;

        // Document coordinates (accounting for scroll)
        let document_y = (relative_y + self.scroll.scroll_offset) as i32;
        let document_x = relative_x as i32;

        // Check if mouse is over TOC area - handle TOC scrolling if so
        if self.show_toc {
            if let Some(toc_area) = self.calculate_toc_area(area) {
                let is_over_toc = event.column >= toc_area.x
                    && event.column < toc_area.x + toc_area.width
                    && event.row >= toc_area.y
                    && event.row < toc_area.y + toc_area.height;

                if is_over_toc {
                    match event.kind {
                        MouseEventKind::Moved => {
                            self.handle_toc_hover_internal(event, toc_area);
                            return MarkdownEvent::None;
                        }
                        MouseEventKind::Down(MouseButton::Left) => {
                            if self.handle_toc_click_internal(event, toc_area) {
                                return MarkdownEvent::None;
                            }
                            return MarkdownEvent::None;
                        }
                        MouseEventKind::ScrollUp => {
                            self.toc_scroll_offset = self.toc_scroll_offset.saturating_sub(1);
                            self.update_toc_hovered_entry(event.column, event.row, toc_area);
                            return MarkdownEvent::None;
                        }
                        MouseEventKind::ScrollDown => {
                            let entry_count = self
                                .toc_state
                                .as_ref()
                                .map(|s| s.entry_count())
                                .unwrap_or(0);
                            let visible_height = toc_area.height as usize;
                            let max_offset = entry_count.saturating_sub(visible_height);
                            if self.toc_scroll_offset < max_offset {
                                self.toc_scroll_offset += 1;
                            }
                            self.update_toc_hovered_entry(event.column, event.row, toc_area);
                            return MarkdownEvent::None;
                        }
                        _ => {}
                    }
                } else if matches!(event.kind, MouseEventKind::Moved) {
                    self.toc_hovered = false;
                    self.toc_hovered_entry = None;
                }
            }
        }

        // Check if click is on scrollbar area (rightmost column(s) of content area)
        if let Some(scrollbar_area) = self.calculate_scrollbar_area(area) {
            if is_in_scrollbar_area(event.column, event.row, scrollbar_area) {
                match event.kind {
                    MouseEventKind::Down(MouseButton::Left)
                    | MouseEventKind::Drag(MouseButton::Left) => {
                        // Click or drag on scrollbar - jump to position
                        let new_offset = click_to_offset(event.row, scrollbar_area, &self.scroll);
                        self.scroll.scroll_offset = new_offset;
                        return MarkdownEvent::Scrolled {
                            offset: new_offset,
                            direction: 0,
                        };
                    }
                    MouseEventKind::ScrollUp => {
                        let old_offset = self.scroll.scroll_offset;
                        self.scroll.scroll_up(5);
                        return MarkdownEvent::Scrolled {
                            offset: self.scroll.scroll_offset,
                            direction: -(old_offset.saturating_sub(self.scroll.scroll_offset)
                                as i32),
                        };
                    }
                    MouseEventKind::ScrollDown => {
                        let old_offset = self.scroll.scroll_offset;
                        self.scroll.scroll_down(5);
                        return MarkdownEvent::Scrolled {
                            offset: self.scroll.scroll_offset,
                            direction: (self.scroll.scroll_offset.saturating_sub(old_offset)
                                as i32),
                        };
                    }
                    _ => {}
                }
            }
        }

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                // Exit active selection on new click
                if self.selection.is_active() {
                    self.selection.exit();
                }

                // Process click for double-click detection
                // Pass current scroll_offset so it can be stored for accurate line calculation later
                let (is_double, _should_process_pending) = self.double_click.process_click(
                    event.column,
                    event.row,
                    self.scroll.scroll_offset,
                );

                if is_double {
                    // Double-click: store info for app to retrieve, return None
                    if let Some(evt) = self.get_line_info_at_position(relative_y, width) {
                        self.last_double_click = Some((evt.0, evt.1, evt.2));
                    }
                    return MarkdownEvent::None;
                }

                // Single click: highlight the clicked line (set as current line)
                let clicked_line = self.scroll.scroll_offset + relative_y + 1; // 1-indexed
                if clicked_line <= self.scroll.total_lines {
                    self.scroll.set_current_line(clicked_line);
                }

                MarkdownEvent::FocusedLine { line: clicked_line }
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                let event_result = if !self.selection.is_active() {
                    // Start selection on drag
                    self.selection.enter(
                        document_x,
                        document_y,
                        self.rendered_lines.clone(),
                        width,
                    );
                    self.selection.anchor = Some(SelectionPos::new(document_x, document_y));
                    self.mode = MarkdownWidgetMode::Drag;
                    MarkdownEvent::SelectionStarted
                } else {
                    MarkdownEvent::None
                };

                // Update cursor position during drag
                self.selection.update_cursor(document_x, document_y);

                event_result
            }
            MouseEventKind::Up(MouseButton::Left) => {
                // Selection complete - auto-copy to clipboard
                if self.selection.is_active() && self.selection.has_selection() {
                    // Update frozen lines with current rendered lines
                    self.selection.frozen_lines = Some(self.rendered_lines.clone());
                    self.selection.frozen_width = width;

                    // Auto-copy to clipboard
                    if let Some(text) = self.selection.get_selected_text() {
                        if !text.is_empty() {
                            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                                if clipboard.set_text(&text).is_ok() {
                                    // Store in selection state for app to retrieve (shows toast)
                                    self.selection.last_copied_text = Some(text.clone());
                                    return MarkdownEvent::Copied { text };
                                }
                            }
                        }
                    }
                }
                MarkdownEvent::None
            }
            MouseEventKind::ScrollUp => {
                let old_offset = self.scroll.scroll_offset;
                self.scroll.scroll_up(5);
                MarkdownEvent::Scrolled {
                    offset: self.scroll.scroll_offset,
                    direction: -(old_offset.saturating_sub(self.scroll.scroll_offset) as i32),
                }
            }
            MouseEventKind::ScrollDown => {
                let old_offset = self.scroll.scroll_offset;
                self.scroll.scroll_down(5);
                MarkdownEvent::Scrolled {
                    offset: self.scroll.scroll_offset,
                    direction: (self.scroll.scroll_offset.saturating_sub(old_offset) as i32),
                }
            }
            _ => MarkdownEvent::None,
        };

        self.check_pending_click_internal(area)
    }

    fn handle_toc_hover_internal(&mut self, event: &MouseEvent, toc_area: Rect) {
        let _prev_hovered = self.toc_hovered;
        let _prev_entry = self.toc_hovered_entry;

        let auto_state = TocState::from_content(&self.content);
        let toc_state = if let Some(provided) = &self.toc_state {
            if provided.entries.is_empty() {
                &auto_state
            } else {
                provided
            }
        } else {
            &auto_state
        };

        let toc = Toc::new(toc_state)
            .expanded(self.toc_hovered)
            .config(self.toc_config.clone());

        let entry = toc.entry_at_position(event.column, event.row, toc_area);

        if entry.is_some() {
            self.toc_hovered = true;
            self.toc_hovered_entry = entry;
        } else {
            self.toc_hovered = false;
            self.toc_hovered_entry = None;
        }
    }

    fn handle_toc_click_internal(&mut self, event: &MouseEvent, toc_area: Rect) -> bool {
        let auto_state = TocState::from_content(&self.content);
        let toc_state = if let Some(provided) = &self.toc_state {
            if provided.entries.is_empty() {
                &auto_state
            } else {
                provided
            }
        } else {
            &auto_state
        };

        let toc = Toc::new(toc_state)
            .expanded(self.toc_hovered)
            .config(self.toc_config.clone());

        if let Some(entry_idx) = toc.entry_at_position(event.column, event.row, toc_area) {
            if let Some(target_line) = toc.click_to_line(entry_idx) {
                let new_offset = target_line.saturating_sub(2);
                let max_offset = self
                    .scroll
                    .total_lines
                    .saturating_sub(self.scroll.viewport_height);
                self.scroll.scroll_offset = new_offset.min(max_offset);
                self.scroll.current_line = target_line.saturating_add(1);
                self.toc_hovered_entry = Some(entry_idx);
                return true;
            }
        }
        false
    }

    fn check_pending_click_internal(&mut self, area: Rect) -> MarkdownEvent {
        if let Some((x, y, click_scroll_offset)) = self.double_click.check_pending_timeout() {
            let relative_y = y.saturating_sub(area.y) as usize;
            let relative_x = x.saturating_sub(area.x) as usize;
            let width = area.width as usize;

            let clicked_line = click_scroll_offset + relative_y + 1;
            if clicked_line <= self.scroll.total_lines {
                self.scroll.set_current_line(clicked_line);
            }

            if self.handle_click_collapse(relative_x, relative_y, width) {
                if let Some((_, line_kind, text)) =
                    self.get_line_info_at_position(relative_y, width)
                {
                    if line_kind == "Heading" {
                        return MarkdownEvent::HeadingToggled {
                            level: 1,
                            text,
                            collapsed: true,
                        };
                    }
                }
            }

            return MarkdownEvent::FocusedLine { line: clicked_line };
        }

        MarkdownEvent::None
    }

    /// Handle click for collapse/expand functionality.
    ///
    /// Returns `true` if a collapsible element was toggled.
    fn handle_click_collapse(&mut self, _x: usize, y: usize, width: usize) -> bool {
        use crate::widgets::markdown_widget::foundation::elements::ElementKind;

        let elements = render_markdown_to_elements(&self.content, true);

        // Account for scroll offset - y is relative to visible area
        let document_y = y + self.scroll.scroll_offset;
        let mut line_idx = 0;

        for (idx, element) in elements.iter().enumerate() {
            // Skip elements that shouldn't be rendered (collapsed sections)
            if !should_render_line(element, idx, &self.collapse) {
                continue;
            }

            let rendered = render(element, width);
            let line_count = rendered.len();

            if document_y >= line_idx && document_y < line_idx + line_count {
                match &element.kind {
                    ElementKind::Heading { section_id, .. } => {
                        // Only collapse headings if show_heading_collapse is enabled
                        if self.display.show_heading_collapse {
                            self.collapse.toggle_section(*section_id);
                            self.cache.invalidate();
                            return true;
                        }
                    }
                    ElementKind::Frontmatter { .. } => {
                        self.collapse.toggle_section(0);
                        self.cache.invalidate();
                        return true;
                    }
                    ElementKind::FrontmatterStart { .. } => {
                        self.collapse.toggle_section(0);
                        self.cache.invalidate();
                        return true;
                    }
                    ElementKind::ExpandToggle { content_id, .. } => {
                        self.expandable.toggle(content_id);
                        self.cache.invalidate();
                        return true;
                    }
                    _ => {}
                }
            }

            line_idx += line_count;
        }

        false
    }

    /// Get line information at a given screen position.
    ///
    /// Returns (line_number, line_kind, content) if found.
    pub fn get_line_info_at_position(
        &self,
        y: usize,
        width: usize,
    ) -> Option<(usize, String, String)> {
        use crate::widgets::markdown_widget::foundation::elements::ElementKind;

        let elements = render_markdown_to_elements(&self.content, true);
        let document_y = y + self.scroll.scroll_offset;
        let mut visual_line_idx = 0;
        let mut logical_line_num = 0;

        for (idx, element) in elements.iter().enumerate() {
            if !should_render_line(element, idx, &self.collapse) {
                continue;
            }

            logical_line_num += 1;

            let rendered = render(element, width);
            let line_count = rendered.len();

            if document_y >= visual_line_idx && document_y < visual_line_idx + line_count {
                let line_kind = match &element.kind {
                    ElementKind::Heading { .. } => "Heading",
                    ElementKind::Paragraph(_) => "Paragraph",
                    ElementKind::CodeBlockHeader { .. } => "CodeBlockHeader",
                    ElementKind::CodeBlockContent { .. } => "CodeBlockContent",
                    ElementKind::CodeBlockBorder { .. } => "CodeBlockBorder",
                    ElementKind::ListItem { .. } => "ListItem",
                    ElementKind::Blockquote { .. } => "Blockquote",
                    ElementKind::Empty => "Empty",
                    ElementKind::HorizontalRule => "HorizontalRule",
                    ElementKind::Frontmatter { .. } => "Frontmatter",
                    ElementKind::FrontmatterStart { .. } => "FrontmatterStart",
                    ElementKind::FrontmatterField { .. } => "FrontmatterField",
                    ElementKind::FrontmatterEnd => "FrontmatterEnd",
                    ElementKind::Expandable { .. } => "Expandable",
                    ElementKind::ExpandToggle { .. } => "ExpandToggle",
                    ElementKind::TableRow { .. } => "TableRow",
                    ElementKind::TableBorder(_) => "TableBorder",
                    ElementKind::HeadingBorder { .. } => "HeadingBorder",
                };

                let text_content = self.get_element_text(&element.kind);

                return Some((logical_line_num, line_kind.to_string(), text_content));
            }

            visual_line_idx += line_count;
        }

        None
    }

    /// Extract plain text from an ElementKind.
    fn get_element_text(
        &self,
        kind: &crate::widgets::markdown_widget::foundation::elements::ElementKind,
    ) -> String {
        use crate::widgets::markdown_widget::foundation::elements::{ElementKind, TextSegment};

        fn segment_to_text(seg: &TextSegment) -> &str {
            match seg {
                TextSegment::Plain(s) => s,
                TextSegment::Bold(s) => s,
                TextSegment::Italic(s) => s,
                TextSegment::BoldItalic(s) => s,
                TextSegment::InlineCode(s) => s,
                TextSegment::Link { text, .. } => text,
                TextSegment::Strikethrough(s) => s,
                TextSegment::Html(s) => s,
                TextSegment::Checkbox(_) => "",
            }
        }

        match kind {
            ElementKind::Heading { text, .. } => text.iter().map(segment_to_text).collect(),
            ElementKind::Paragraph(segments) => segments.iter().map(segment_to_text).collect(),
            ElementKind::CodeBlockContent { content, .. } => content.clone(),
            ElementKind::CodeBlockHeader { language, .. } => language.clone(),
            ElementKind::ListItem { content, .. } => content.iter().map(segment_to_text).collect(),
            ElementKind::Blockquote { content, .. } => {
                content.iter().map(segment_to_text).collect()
            }
            ElementKind::Frontmatter { fields, .. } => fields
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<_>>()
                .join(", "),
            ElementKind::FrontmatterField { key, value } => format!("{}: {}", key, value),
            ElementKind::TableRow { cells, .. } => cells.join(" | "),
            _ => String::new(),
        }
    }

    /// Set the rendered lines for selection text extraction.
    ///
    /// Call this after rendering to update the cached lines.
    pub fn set_rendered_lines(&mut self, lines: Vec<ratatui::text::Line<'static>>) {
        self.rendered_lines = lines;
    }

    /// Check if selection mode is active.
    pub fn is_selection_active(&self) -> bool {
        self.selection.is_active()
    }

    /// Get the current selection state (for rendering).
    pub fn selection(
        &self,
    ) -> &crate::widgets::markdown_widget::state::selection_state::SelectionState {
        &self.selection
    }

    /// Get line information at the current highlighted line.
    ///
    /// Returns (line_number, line_kind, content) if found.
    pub fn get_current_line_info(&self, width: usize) -> Option<(usize, String, String)> {
        // current_line is 1-indexed document line, get_line_info_at_position expects
        // a relative viewport position, so we need to convert.
        // The document position of current_line is current_line - 1 (0-indexed).
        // Since get_line_info_at_position adds scroll_offset, we pass (current_line - 1).
        let document_y = self.scroll.current_line.saturating_sub(1);
        let elements = render_markdown_to_elements(&self.content, true);
        let mut visual_line_idx = 0;
        let mut logical_line_num = 0;

        for (idx, element) in elements.iter().enumerate() {
            if !should_render_line(element, idx, &self.collapse) {
                continue;
            }

            logical_line_num += 1;

            let rendered = render(element, width);
            let line_count = rendered.len();

            if document_y >= visual_line_idx && document_y < visual_line_idx + line_count {
                let line_kind = match &element.kind {
                    ElementKind::Heading { .. } => "Heading",
                    ElementKind::Paragraph(_) => "Paragraph",
                    ElementKind::CodeBlockHeader { .. } => "CodeBlockHeader",
                    ElementKind::CodeBlockContent { .. } => "CodeBlockContent",
                    ElementKind::CodeBlockBorder { .. } => "CodeBlockBorder",
                    ElementKind::ListItem { .. } => "ListItem",
                    ElementKind::Blockquote { .. } => "Blockquote",
                    ElementKind::Empty => "Empty",
                    ElementKind::HorizontalRule => "HorizontalRule",
                    ElementKind::Frontmatter { .. } => "Frontmatter",
                    ElementKind::FrontmatterStart { .. } => "FrontmatterStart",
                    ElementKind::FrontmatterField { .. } => "FrontmatterField",
                    ElementKind::FrontmatterEnd => "FrontmatterEnd",
                    ElementKind::Expandable { .. } => "Expandable",
                    ElementKind::ExpandToggle { .. } => "ExpandToggle",
                    ElementKind::TableRow { .. } => "TableRow",
                    ElementKind::TableBorder(_) => "TableBorder",
                    ElementKind::HeadingBorder { .. } => "HeadingBorder",
                };

                let text_content = self.get_element_text(&element.kind);

                return Some((logical_line_num, line_kind.to_string(), text_content));
            }

            visual_line_idx += line_count;
        }

        None
    }
}
