/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::HTMLTimeElementBinding;
use dom::bindings::codegen::InheritTypes::HTMLTimeElementDerived;
use dom::bindings::js::{JSRef, Temporary};
use dom::bindings::utils::{Reflectable, Reflector};
use dom::document::Document;
use dom::element::HTMLTimeElementTypeId;
use dom::eventtarget::{EventTarget, NodeTargetTypeId};
use dom::htmlelement::HTMLElement;
use dom::node::{Node, ElementNodeTypeId};
use servo_util::str::DOMString;

#[deriving(Encodable)]
pub struct HTMLTimeElement {
    pub htmlelement: HTMLElement
}

impl HTMLTimeElementDerived for EventTarget {
    fn is_htmltimeelement(&self) -> bool {
        self.type_id == NodeTargetTypeId(ElementNodeTypeId(HTMLTimeElementTypeId))
    }
}

impl HTMLTimeElement {
    pub fn new_inherited(localName: DOMString, document: &JSRef<Document>) -> HTMLTimeElement {
        HTMLTimeElement {
            htmlelement: HTMLElement::new_inherited(HTMLTimeElementTypeId, localName, document)
        }
    }

    pub fn new(localName: DOMString, document: &JSRef<Document>) -> Temporary<HTMLTimeElement> {
        let element = HTMLTimeElement::new_inherited(localName, document);
        Node::reflect_node(box element, document, HTMLTimeElementBinding::Wrap)
    }
}

impl Reflectable for HTMLTimeElement {
    fn reflector<'a>(&'a self) -> &'a Reflector {
        self.htmlelement.reflector()
    }
}
