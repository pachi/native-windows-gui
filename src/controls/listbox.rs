/*!
    Simple listbox control definition
*/
/*
    Copyright (C) 2016  Gabriel Dubé

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::hash::Hash;
use std::any::TypeId;
use std::fmt::Display;
use std::mem;

use winapi::{HWND, HFONT};

use ui::Ui;
use controls::{Control, ControlT, AnyHandle};
use error::Error;
use events::Event;
use low::other_helper::to_utf16;

/**
    Template that creates a listbox control

    Members:  
    • `collection`: Item collection of the listbox. The item type must implement Display  
    • `position`: The start position of the listbox  
    • `size`: The start size of the listbox  
    • `visible`: If the listbox should be visible to the user32  
    • `disabled`: If the user can or can't use the listbox  
    • `readonly` : If true, the user won't be able to select items in the listbox
    • `multi_select`: If true, allow the user to select more than one item  
    • `parent`: The listbox parent  
    • `font`: The listbox font. If None, use the system default  
*/
#[derive(Clone)]
pub struct ListBoxT<D: Clone+Display+'static, ID: Hash+Clone> {
    pub collection: Vec<D>,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub visible: bool,
    pub disabled: bool,
    pub readonly: bool,
    pub multi_select: bool,
    pub parent: ID,
    pub font: Option<ID>,
}

impl<D: Clone+Display+'static, ID: Hash+Clone> ControlT<ID> for ListBoxT<D, ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<ListBox<D>>() }

    fn events(&self) -> Vec<Event> {
        vec![Event::Destroyed]
    }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        use low::window_helper::{WindowParams, build_window, set_window_font, handle_of_window, handle_of_font};
        use low::defs::{LB_ADDSTRING, LBS_HASSTRINGS, LBS_MULTIPLESEL, LBS_NOSEL};
        use winapi::{DWORD, WS_VISIBLE, WS_DISABLED, WS_CHILD, WS_BORDER, WS_VSCROLL, WS_HSCROLL};
        use user32::{SendMessageW};

        let flags: DWORD = WS_CHILD | WS_BORDER | LBS_HASSTRINGS | WS_VSCROLL | WS_HSCROLL |
        if self.visible      { WS_VISIBLE }      else { 0 } |
        if self.disabled     { WS_DISABLED }     else { 0 } |
        if self.multi_select { LBS_MULTIPLESEL } else { 0 } |
        if self.readonly     { LBS_NOSEL }       else { 0 };

        // Get the parent handle
        let parent = match handle_of_window(ui, &self.parent, "The parent of a listbox must be a window-like control.") {
            Ok(h) => h,
            Err(e) => { return Err(e); }
        };

        // Get the font handle (if any)
        let font_handle: Option<HFONT> = match self.font.as_ref() {
            Some(font_id) => 
                match handle_of_font(ui, &font_id, "The font of a listbox must be a font resource.") {
                    Ok(h) => Some(h),
                    Err(e) => { return Err(e); }
                },
            None => None
        };

        let params = WindowParams {
            title: "",
            class_name: "LISTBOX",
            position: self.position.clone(),
            size: self.size.clone(),
            flags: flags,
            parent: parent
        };

        match unsafe{ build_window(params) } {
            Ok(h) => {
                unsafe{ 
                    set_window_font(h, font_handle, true); 
                    let collection: Vec<D> = self.collection.iter().map(
                        |s|{  
                            let text = to_utf16(format!("{}", s).as_str());
                            SendMessageW(h, LB_ADDSTRING, 0, mem::transmute(text.as_ptr()));
                            s.clone() 
                        } 
                    ).collect();
                    Ok( Box::new(ListBox{handle: h, collection: collection}) )
                }
            },
            Err(e) => Err(Error::System(e))
        }
    }
}

/**
    A listbox control
*/
pub struct ListBox<D: Clone+Display> {
    handle: HWND,
    collection: Vec<D>
}

impl<D: Clone+Display> ListBox<D> {

    /// Return the inner collection of the listbox
    pub fn collection(&self) -> &Vec<D> { &self.collection }

    /// Return the inner collection of the listbox, mutable.
    /// If the inner listbox is changed, `listbox.sync` must be called to show the changes in the listbox
    pub fn collection_mut(&mut self) -> &mut Vec<D> { &mut self.collection }

    /// Reload the content of the listbox
    pub fn sync(&self) {
        use low::defs::{LB_RESETCONTENT, LB_ADDSTRING};
        use user32::SendMessageW;

        unsafe{ SendMessageW(self.handle, LB_RESETCONTENT, 0, 0); }

        for i in self.collection.iter() {
            let text = to_utf16(format!("{}", i).as_str());
            unsafe{ SendMessageW(self.handle, LB_ADDSTRING, 0, mem::transmute(text.as_ptr())); }
        }
    }

    /// Add an item at the end of the listbox. Updates both the inner collection and the ui.
    pub fn push(&mut self, item: D) {
        use user32::SendMessageW;
        use low::defs::LB_ADDSTRING;

        let text = to_utf16(format!("{}", item).as_str());
        unsafe{ SendMessageW(self.handle, LB_ADDSTRING, 0, mem::transmute(text.as_ptr())); }

        self.collection.push(item);
    }

    /// Insert an item at the selected position in the lisbox and the inner collection.  
    /// If index is -1, the item is added at the end of the list.
    pub fn insert(&mut self, index: usize, item: D) {
        use user32::SendMessageW;
        use low::defs::LB_INSERTSTRING;
        use winapi::WPARAM;

        let text = to_utf16(format!("{}", item).as_str());
        unsafe{ SendMessageW(self.handle, LB_INSERTSTRING, index as WPARAM, mem::transmute(text.as_ptr())); }

        self.collection.insert(index, item);
    }

    /// Return the index of currently selected item.  
    /// Return None if there are no item selected  
    /// If the listbox can have more than one selected item, use `get_selected_indexes`
    pub fn get_selected_index(&self) -> Option<usize> {
        use user32::SendMessageW;
        use low::defs::LB_GETCURSEL;

        let index = unsafe{ SendMessageW(self.handle, LB_GETCURSEL, 0, 0) };
        if index == -1 { None } 
        else { Some(index as usize) }
    }

    /// Return a vector filled with the selected indexes of the listbox.
    /// If nothing is selected or the listbox do not support multiple selection, the returned vector will be empty.
    pub fn get_selected_indexes(&self) -> Vec<usize> {
        use user32::SendMessageW;
        use low::defs::{LB_GETSELCOUNT, LB_GETSELITEMS};
        use winapi::WPARAM;

        let selected_count = unsafe{ SendMessageW(self.handle, LB_GETSELCOUNT, 0, 0) };
        if selected_count == 0 || selected_count == -1 {
            return Vec::new();
        } 

        unsafe{ 
            let mut buffer: Vec<u32> = Vec::with_capacity(selected_count as usize);
            buffer.set_len(selected_count as usize);
            SendMessageW(self.handle, LB_GETSELITEMS, selected_count as WPARAM, mem::transmute(buffer.as_mut_ptr()) );
            buffer.into_iter().map(|i| i as usize).collect()
        }
    }

    /// Return true if `index` is currently selected in the listbox
    pub fn index_selected(&self, index: usize) -> bool {
        use user32::SendMessageW;
        use low::defs::LB_GETSEL;
        use winapi::WPARAM;

       unsafe{ SendMessageW(self.handle, LB_GETSEL, index as WPARAM, 0) > 0 }
    }

    /// Set the selected index in a single choice listbox.  
    /// For multi-select listbox use `set_index_selected` or `set_range_selected`  
    /// If `index` is -1, remove the selected index from the listbox
    pub fn set_current_index(&self, index: usize) {
        use user32::SendMessageW;
        use low::defs::LB_SETCURSEL;
        use winapi::WPARAM;
        unsafe{ SendMessageW(self.handle, LB_SETCURSEL, index as WPARAM, 0); }
    }

    /// Set the selected state of the item located at index. Only work for multi-select listbox
    /// For single listbox, use `set_current_index`
    /// If index is -1, the change is applied to every item.
    pub fn set_index_selected(&self, index: usize, selected: bool) {
        use user32::SendMessageW;
        use low::defs::LB_SETSEL;
        use winapi::{WPARAM, LPARAM};

        let selected: WPARAM = (selected == true) as WPARAM;
        unsafe { SendMessageW(self.handle, LB_SETSEL, selected, index as LPARAM); }
    }

    /// Return the number of selected items
    pub fn len_selected(&self) -> usize {
        use user32::SendMessageW;
        use low::defs::LB_GETSELCOUNT;
        unsafe{ SendMessageW(self.handle, LB_GETSELCOUNT, 0, 0) as usize }
    }

    /// Remove every item in the inner collection and in the listbox
    pub fn clear(&mut self) {
        use user32::SendMessageW;
        use low::defs::LB_RESETCONTENT;

        unsafe{ SendMessageW(self.handle, LB_RESETCONTENT, 0, 0) };
        self.collection.clear();
    }

    /// Return the number of items in the inner collection
    pub fn len(&self) -> usize { self.collection.len() }

    pub fn get_visibility(&self) -> bool { unsafe{ ::low::window_helper::get_window_visibility(self.handle) } }
    pub fn set_visibility(&self, visible: bool) { unsafe{ ::low::window_helper::set_window_visibility(self.handle, visible); }}
    pub fn get_position(&self) -> (i32, i32) { unsafe{ ::low::window_helper::get_window_position(self.handle) } }
    pub fn set_position(&self, x: i32, y: i32) { unsafe{ ::low::window_helper::set_window_position(self.handle, x, y); }}
    pub fn get_size(&self) -> (u32, u32) { unsafe{ ::low::window_helper::get_window_size(self.handle) } }
    pub fn set_size(&self, w: u32, h: u32) { unsafe{ ::low::window_helper::set_window_size(self.handle, w, h, true); } }
    pub fn get_enabled(&self) -> bool { unsafe{ ::low::window_helper::get_window_enabled(self.handle) } }
    pub fn set_enabled(&self, e:bool) { unsafe{ ::low::window_helper::set_window_enabled(self.handle, e); } }
}


impl<D: Clone+Display> Control for ListBox<D> {

    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

    fn free(&mut self) {
        use user32::DestroyWindow;
        unsafe{ DestroyWindow(self.handle) };
    }

}