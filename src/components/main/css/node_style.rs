/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// Style retrieval from DOM elements.

use css::node_util::NodeUtil;
use layout::incremental::RestyleDamage;

use newcss::complete::CompleteStyle;
use script::dom::node::{AbstractNode, LayoutView};
use servo_util::tree::TreeNodeRef;


/// Node mixin providing `style` method that returns a `NodeStyle`
pub trait StyledNode {
    fn style(&self) -> CompleteStyle;
    fn restyle_damage(&self) -> RestyleDamage;
}

impl StyledNode for AbstractNode<LayoutView> {
    fn style(&self) -> CompleteStyle {
        assert!(self.is_element()); // Only elements can have styles
        let results = self.get_css_select_results();
        results.computed_style()
    }

    fn restyle_damage(&self) -> RestyleDamage {
        self.get_restyle_damage()
    }
}
