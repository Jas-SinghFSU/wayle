mod factory;
mod messages;
mod styling;
mod watchers;

use std::sync::Arc;

use gtk::prelude::*;
use relm4::prelude::*;
use wayle_config::ConfigService;

pub(crate) use self::{
    factory::Factory,
    messages::{SeparatorCmd, SeparatorInit},
};

/// Visual separator displayed between bar modules.
pub(crate) struct SeparatorModule {
    separator: gtk::Separator,
    css_provider: gtk::CssProvider,
    is_vertical: bool,
    config: Arc<ConfigService>,
}

#[relm4::component(pub(crate))]
impl Component for SeparatorModule {
    type Init = SeparatorInit;
    type Input = ();
    type Output = ();
    type CommandOutput = SeparatorCmd;

    view! {
        gtk::Box {
            add_css_class: "separator",
            set_expand: true,
            set_align: gtk::Align::Center,

            #[local_ref]
            separator -> gtk::Separator {
                #[watch]
                set_orientation: Self::orientation_for_vertical(model.is_vertical),
            }
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let is_vertical = init.is_vertical.get();
        let separator = gtk::Separator::new(Self::orientation_for_vertical(is_vertical));

        let model = Self {
            separator: separator.clone(),
            css_provider: gtk::CssProvider::new(),
            is_vertical,
            config: init.config.clone(),
        };
        let widgets = view_output!();

        styling::init_css_provider(&model.separator, &model.css_provider);
        styling::apply_styling(&model.css_provider, model.is_vertical, &init.config);
        watchers::spawn_watchers(&sender, init.is_vertical, &init.config);

        ComponentParts { model, widgets }
    }

    fn update_cmd(
        &mut self,
        msg: SeparatorCmd,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            SeparatorCmd::StylingChanged => {
                styling::apply_styling(&self.css_provider, self.is_vertical, &self.config);
            }
            SeparatorCmd::OrientationChanged(is_vertical) => {
                self.is_vertical = is_vertical;
                styling::apply_styling(&self.css_provider, self.is_vertical, &self.config);
            }
        }
    }
}

impl SeparatorModule {
    /// Returns the separator orientation based on bar orientation.
    fn orientation_for_vertical(is_vertical: bool) -> gtk::Orientation {
        if is_vertical {
            gtk::Orientation::Horizontal
        } else {
            gtk::Orientation::Vertical
        }
    }
}
