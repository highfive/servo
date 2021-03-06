/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! DOM bindings for `CharacterData`.

use dom::bindings::utils::{DOMString, ErrorResult, Fallible};
use dom::bindings::utils::{Reflectable, Reflector};
use dom::document::AbstractDocument;
use dom::node::{Node, NodeTypeId, ScriptView};
use js::jsapi::{JSObject, JSContext};

pub struct CharacterData {
    node: Node<ScriptView>,
    data: ~str
}

impl CharacterData {
    pub fn new(id: NodeTypeId, data: ~str, document: AbstractDocument) -> CharacterData {
        CharacterData {
            node: Node::new(id, document),
            data: data
        }
    }
    
    pub fn Data(&self) -> DOMString {
        Some(self.data.clone())
    }

    pub fn SetData(&mut self, arg: &DOMString) -> ErrorResult {
        self.data = arg.get_ref().clone();
        Ok(())
    }

    pub fn Length(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn SubstringData(&self, offset: u32, count: u32) -> Fallible<DOMString> {
        Ok(Some(self.data.slice(offset as uint, count as uint).to_str()))
    }

    pub fn AppendData(&mut self, arg: &DOMString) -> ErrorResult {
        self.data.push_str(arg.get_ref().clone());
        Ok(())
    }

    pub fn InsertData(&mut self, _offset: u32, _arg: &DOMString) -> ErrorResult {
        fail!("CharacterData::InsertData() is unimplemented")
    }

    pub fn DeleteData(&mut self, _offset: u32, _count: u32) -> ErrorResult {
        fail!("CharacterData::DeleteData() is unimplemented")
    }

    pub fn ReplaceData(&mut self, _offset: u32, _count: u32, _arg: &DOMString) -> ErrorResult {
        fail!("CharacterData::ReplaceData() is unimplemented")
    }
}

impl Reflectable for CharacterData {
    fn reflector<'a>(&'a self) -> &'a Reflector {
        self.node.reflector()
    }

    fn mut_reflector<'a>(&'a mut self) -> &'a mut Reflector {
        self.node.mut_reflector()
    }

    fn wrap_object_shared(@mut self, _cx: *JSContext, _scope: *JSObject) -> *JSObject {
        fail!(~"need to implement wrapping");
    }

    fn GetParentObject(&self, cx: *JSContext) -> Option<@mut Reflectable> {
        self.node.GetParentObject(cx)
    }
}
