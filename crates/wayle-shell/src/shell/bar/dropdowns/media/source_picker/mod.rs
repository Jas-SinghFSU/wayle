mod messages;
mod methods;
mod source_item;
mod watchers;

use std::sync::Arc;

use gtk::prelude::*;
use relm4::{gtk, prelude::*};
use wayle_media::MediaService;
use wayle_widgets::prelude::*;

pub(crate) use self::messages::*;
use self::source_item::SourceItem;
use crate::i18n::t;

pub(crate) struct SourcePicker {
    media: Arc<MediaService>,
    sources: FactoryVecDeque<SourceItem>,
}

#[relm4::component(pub(crate))]
impl Component for SourcePicker {
    type Init = SourcePickerInit;
    type Input = SourcePickerInput;
    type Output = SourcePickerOutput;
    type CommandOutput = SourcePickerCmd;

    view! {
        #[root]
        gtk::Box {
            add_css_class: "media-source-picker",
            set_orientation: gtk::Orientation::Vertical,

            #[name = "picker_header"]
            gtk::Box {
                add_css_class: "picker-header",

                #[template]
                GhostIconButton {
                    add_css_class: "picker-back",
                    set_icon_name: "ld-arrow-left-symbolic",
                    connect_clicked => SourcePickerInput::BackClicked,
                },

                #[name = "picker_title"]
                gtk::Label {
                    add_css_class: "picker-title",
                    set_label: &t!("dropdown-media-sources"),
                },
            },

            #[name = "source_list_container"]
            gtk::ScrolledWindow {
                add_css_class: "picker-body",
                set_vexpand: true,
                set_hscrollbar_policy: gtk::PolicyType::Never,

                #[local_ref]
                source_list -> gtk::ListBox {
                    add_css_class: "media-source-list",
                    set_activate_on_single_click: true,
                    set_selection_mode: gtk::SelectionMode::None,
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let source_list = gtk::ListBox::new();
        let picker_sender = sender.input_sender().clone();
        source_list.connect_row_activated(move |_list_box, row| {
            if let Ok(index) = usize::try_from(row.index()) {
                picker_sender.emit(SourcePickerInput::SourceSelected(index));
            }
        });

        let sources = FactoryVecDeque::builder().launch(source_list).detach();

        watchers::spawn(&sender, &init.media);

        let model = Self {
            media: init.media,
            sources,
        };

        let source_list = model.sources.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            SourcePickerInput::BackClicked => {
                let _ = sender.output(SourcePickerOutput::NavigateBack);
            }

            SourcePickerInput::SourceSelected(index) => {
                self.select_source(index, &sender);
                let _ = sender.output(SourcePickerOutput::NavigateBack);
            }
        }
    }

    fn update_cmd(
        &mut self,
        msg: SourcePickerCmd,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            SourcePickerCmd::PlayerListChanged { players, active_id } => {
                self.rebuild_source_list(&players, active_id.as_ref());
            }
        }
    }
}
