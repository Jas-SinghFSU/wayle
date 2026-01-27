mod helpers;
mod messages;
mod watchers;

use gtk::prelude::WidgetExt;
use relm4::prelude::*;
use wayle_common::{ConfigProperty, WatcherToken, process, services};
use wayle_config::{
    ConfigService,
    schemas::{modules::MediaIconType, styling::CssToken},
};
use wayle_media::{MediaService, types::PlaybackState};
use wayle_widgets::prelude::{
    BarButton, BarButtonBehavior, BarButtonColors, BarButtonInit, BarButtonInput, BarButtonOutput,
};

pub(crate) use self::messages::{MediaCmd, MediaInit, MediaMsg};

pub(crate) struct MediaModule {
    bar_button: Controller<BarButton>,
    visible: ConfigProperty<bool>,
    active_player_watcher_token: WatcherToken,
}

#[relm4::component(pub(crate))]
impl Component for MediaModule {
    type Init = MediaInit;
    type Input = MediaMsg;
    type Output = ();
    type CommandOutput = MediaCmd;

    view! {
        gtk::Box {
            #[local_ref]
            bar_button -> gtk::MenuButton {},
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let config_service = services::get::<ConfigService>();
        let config = config_service.config();
        let media_config = &config.modules.media;

        let visible = ConfigProperty::new(false);

        let bar_button = BarButton::builder()
            .launch(BarButtonInit {
                icon: media_config.icon_name.get().clone(),
                label: String::from("No media"),
                tooltip: None,
                colors: BarButtonColors {
                    icon_color: media_config.icon_color.clone(),
                    label_color: media_config.label_color.clone(),
                    icon_background: media_config.icon_bg_color.clone(),
                    button_background: media_config.button_bg_color.clone(),
                    border_color: media_config.border_color.clone(),
                    auto_icon_color: CssToken::Blue,
                },
                behavior: BarButtonBehavior {
                    label_max_chars: media_config.label_max_length.clone(),
                    show_icon: media_config.icon_show.clone(),
                    show_label: media_config.label_show.clone(),
                    show_border: media_config.border_show.clone(),
                    visible: visible.clone(),
                },
                settings: init.settings,
            })
            .forward(sender.input_sender(), |output| match output {
                BarButtonOutput::LeftClick => MediaMsg::LeftClick,
                BarButtonOutput::RightClick => MediaMsg::RightClick,
                BarButtonOutput::MiddleClick => MediaMsg::MiddleClick,
                BarButtonOutput::ScrollUp => MediaMsg::ScrollUp,
                BarButtonOutput::ScrollDown => MediaMsg::ScrollDown,
            });

        watchers::spawn_watchers(&sender, media_config);

        let model = Self {
            bar_button,
            visible,
            active_player_watcher_token: WatcherToken::new(),
        };
        let bar_button = model.bar_button.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        let config_service = services::get::<ConfigService>();
        let config = &config_service.config().modules.media;

        let cmd = match msg {
            MediaMsg::LeftClick => config.left_click.get(),
            MediaMsg::RightClick => config.right_click.get(),
            MediaMsg::MiddleClick => config.middle_click.get(),
            MediaMsg::ScrollUp => config.scroll_up.get(),
            MediaMsg::ScrollDown => config.scroll_down.get(),
        };

        process::run_if_set(&cmd);
    }

    fn update_cmd(&mut self, msg: MediaCmd, sender: ComponentSender<Self>, root: &Self::Root) {
        let config_service = services::get::<ConfigService>();
        let media_config = &config_service.config().modules.media;

        match msg {
            MediaCmd::PlayerChanged(player) => {
                self.visible.set(player.is_some());

                let use_disc =
                    player.is_some() && media_config.icon_type.get() == MediaIconType::SpinningDisc;
                Self::update_disc_mode(root, use_disc);

                if let Some(player) = player {
                    let label = helpers::build_label(media_config, &player);
                    self.bar_button.emit(BarButtonInput::SetLabel(label));

                    let icon = helpers::build_icon(media_config, &player);
                    self.bar_button.emit(BarButtonInput::SetIcon(icon));

                    let state = player.playback_state.get();
                    Self::update_spinning_state(root, state);

                    let token = self.active_player_watcher_token.reset();
                    watchers::spawn_player_watchers(&sender, &player, token);
                }
            }
            MediaCmd::MetadataChanged => {
                let media_service = services::get::<MediaService>();
                if let Some(player) = media_service.active_player() {
                    let label = helpers::build_label(media_config, &player);
                    self.bar_button.emit(BarButtonInput::SetLabel(label));
                }
            }
            MediaCmd::PlaybackStateChanged => {
                let media_service = services::get::<MediaService>();
                if let Some(player) = media_service.active_player() {
                    let label = helpers::build_label(media_config, &player);
                    self.bar_button.emit(BarButtonInput::SetLabel(label));
                    let state = player.playback_state.get();
                    Self::update_spinning_state(root, state);
                }
            }
            MediaCmd::UpdateIcon(icon) => {
                self.bar_button.emit(BarButtonInput::SetIcon(icon));
            }
            MediaCmd::IconTypeChanged => {
                let media_service = services::get::<MediaService>();
                let use_disc = media_service.active_player().is_some()
                    && media_config.icon_type.get() == MediaIconType::SpinningDisc;
                Self::update_disc_mode(root, use_disc);

                if let Some(player) = media_service.active_player() {
                    let icon = helpers::build_icon(media_config, &player);
                    self.bar_button.emit(BarButtonInput::SetIcon(icon));

                    let state = player.playback_state.get();
                    Self::update_spinning_state(root, state);
                }
            }
        }
    }
}

impl MediaModule {
    fn update_disc_mode(root: &gtk::Box, enabled: bool) {
        if enabled {
            root.add_css_class("media-disc");
        } else {
            root.remove_css_class("media-disc");
        }
    }

    fn update_spinning_state(root: &gtk::Box, state: PlaybackState) {
        match state {
            PlaybackState::Playing => {
                root.add_css_class("media-spinning");
            }
            PlaybackState::Paused | PlaybackState::Stopped => {
                root.remove_css_class("media-spinning");
            }
        }
    }
}
