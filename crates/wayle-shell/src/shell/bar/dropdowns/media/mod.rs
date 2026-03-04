mod factory;
mod helpers;
mod messages;
pub(crate) mod player_view;
pub(crate) mod source_picker;
mod watchers;

use gtk::prelude::*;
use relm4::{gtk, prelude::*};
use wayle_widgets::prelude::*;

pub(super) use self::factory::Factory;
use self::{
    messages::{MediaDropdownCmd, MediaDropdownInit, MediaDropdownMsg},
    player_view::{PlayerView, PlayerViewInit, PlayerViewOutput},
    source_picker::{SourcePicker, SourcePickerInit, SourcePickerOutput},
};
use crate::shell::bar::dropdowns::scaled_dimension;

const BASE_WIDTH: f32 = 380.0;
const BASE_HEIGHT: f32 = 410.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MediaPage {
    Main,
    Sources,
}

impl MediaPage {
    fn name(self) -> &'static str {
        match self {
            Self::Main => "main",
            Self::Sources => "sources",
        }
    }
}

pub(crate) struct MediaDropdown {
    scaled_width: i32,
    scaled_height: i32,
    active_page: MediaPage,
    player_view: Controller<PlayerView>,
    source_picker: Controller<SourcePicker>,
}

#[relm4::component(pub(crate))]
impl Component for MediaDropdown {
    type Init = MediaDropdownInit;
    type Input = MediaDropdownMsg;
    type Output = ();
    type CommandOutput = MediaDropdownCmd;

    view! {
        #[root]
        gtk::Popover {
            set_css_classes: &["dropdown", "media-dropdown"],
            set_has_arrow: false,
            #[watch]
            set_width_request: model.scaled_width,
            #[watch]
            set_height_request: model.scaled_height,

            #[template]
            Dropdown {
                #[name = "stack"]
                gtk::Stack {
                    set_vexpand: true,
                    set_transition_type: gtk::StackTransitionType::SlideLeftRight,
                    set_transition_duration: 200,
                    #[watch]
                    set_visible_child_name: model.active_page.name(),

                    #[local_ref]
                    add_named[Some("main")] = player_view_widget -> gtk::Box {},

                    #[local_ref]
                    add_named[Some("sources")] = source_picker_widget -> gtk::Box {},
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let player_view = PlayerView::builder()
            .launch(PlayerViewInit {
                media: init.media.clone(),
            })
            .forward(sender.input_sender(), MediaDropdownMsg::PlayerView);

        let source_picker = SourcePicker::builder()
            .launch(SourcePickerInit {
                media: init.media.clone(),
            })
            .forward(sender.input_sender(), MediaDropdownMsg::SourcePicker);

        let scale = init.config.config().styling.scale.get().value();
        watchers::spawn(&sender, &init.config);

        let model = Self {
            scaled_width: scaled_dimension(BASE_WIDTH, scale),
            scaled_height: scaled_dimension(BASE_HEIGHT, scale),
            active_page: MediaPage::Main,
            player_view,
            source_picker,
        };

        let player_view_widget = model.player_view.widget();
        let source_picker_widget = model.source_picker.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            MediaDropdownMsg::PlayerView(PlayerViewOutput::ShowSourcePicker) => {
                self.active_page = MediaPage::Sources;
            }

            MediaDropdownMsg::SourcePicker(SourcePickerOutput::NavigateBack) => {
                self.active_page = MediaPage::Main;
            }
        }
    }

    fn update_cmd(
        &mut self,
        msg: MediaDropdownCmd,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            MediaDropdownCmd::ScaleChanged(scale) => {
                self.scaled_width = scaled_dimension(BASE_WIDTH, scale);
                self.scaled_height = scaled_dimension(BASE_HEIGHT, scale);
            }
        }
    }
}
